use derivative::Derivative;
use serde::export::fmt::Debug;
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::SystemTime;
use std::{
    sync::{atomic::AtomicBool, atomic::Ordering, Arc},
    thread,
    thread::JoinHandle,
    time::Duration,
};
use threadpool::ThreadPool;

#[derive(Clone, Debug)]
pub struct Runtime {
    thread: Arc<JoinHandle<()>>,
    is_run: Arc<AtomicBool>,
    tasks: Arc<RwLock<Tasks>>,
}

#[derive(Debug)]
pub struct Tasks {
    tasks: HashMap<u128, Task>,
    next_task: Option<NextTask>,
    counter: u128,
}

#[derive(Debug)]
struct NextTask {
    task_time: u128,
    descriptor: u128,
}

impl Tasks {
    pub fn empty() -> Tasks {
        Tasks {
            tasks: HashMap::new(),
            next_task: None,
            counter: 0,
        }
    }

    pub fn create_task(
        &mut self,
        action: Action,
        interval: Duration,
        is_async: bool,
        is_regular: bool,
    ) -> u128 {
        self.counter += 1;
        let descriptor = self.counter;
        self.tasks.insert(
            descriptor,
            Task::new(interval, action, is_async, is_regular),
        );
        self.compute_next_task();
        descriptor
    }

    pub fn remove_task(&mut self, descriptor: u128) {
        self.tasks.remove(&descriptor);
    }

    pub fn reset_task_time(&mut self, descriptor: u128) {
        if let Some(task) = self.tasks.get_mut(&descriptor) {
            task.last_run = time_ms();
            self.compute_next_task();
        }
    }

    fn run_task(&mut self, task_index: u128, pool: &ThreadPool) {
        let remove_task = {
            if let Some(task) = &mut self.tasks.get_mut(&task_index) {
                task.run(pool);
                !task.is_regular
            } else {
                false
            }
        };

        if remove_task {
            self.tasks.remove(&task_index);
        }

        self.compute_next_task();
    }

    fn compute_next_task(&mut self) {
        let mut next_time = u128::max_value();
        let mut descriptor = None;
        for (index, task) in self.tasks.iter() {
            if next_time > task.next_run() {
                next_time = task.next_run();
                descriptor = Some(index);
            }
        }

        if let Some(descriptor) = descriptor {
            self.next_task = Some(NextTask {
                task_time: next_time,
                descriptor: *descriptor,
            });
        }
    }
}

impl Runtime {
    pub fn new(threads_count: usize) -> Runtime {
        let is_run = Arc::new(AtomicBool::new(true));
        let tasks = Arc::new(RwLock::new(Tasks::empty()));

        let is_run_service = is_run.clone();
        let tasks_service = tasks.clone();
        let thread = Arc::new(thread::spawn(move || {
            Self::run(tasks_service, is_run_service, threads_count)
        }));

        Runtime {
            thread,
            is_run,
            tasks,
        }
    }

    pub fn create_task(
        &self,
        action: Action,
        interval: Duration,
        is_async: bool,
        is_regular: bool,
    ) -> u128 {
        let mut tasks = self.tasks.write().unwrap();
        tasks.create_task(action, interval, is_async, is_regular)
    }

    pub fn remove_task(&self, descriptor: u128) {
        let mut tasks = self.tasks.write().unwrap();
        tasks.remove_task(descriptor)
    }

    pub fn reset_task_time(&self, descriptor: u128) {
        let mut tasks = self.tasks.write().unwrap();
        tasks.reset_task_time(descriptor)
    }

    fn run(tasks: Arc<RwLock<Tasks>>, is_run: Arc<AtomicBool>, threads_count: usize) {
        let pool = ThreadPool::new(threads_count);
        while is_run.load(Ordering::Relaxed) {
            let task_index = {
                let tasks = tasks.read().unwrap();
                if let Some(task) = &tasks.next_task {
                    if task.task_time <= time_ms() {
                        Some(task.descriptor)
                    } else {
                        None
                    }
                } else {
                    None
                }
            };

            if let Some(task_index) = task_index {
                let mut tasks = tasks.write().unwrap();
                tasks.run_task(task_index, &pool);
            } else {
                thread::sleep(Duration::from_millis(500));
            }
        }
    }
}

pub type Action = Arc<Box<dyn Fn() + Send + Sync + 'static>>;

#[derive(Derivative)]
#[derivative(Debug)]
struct Task {
    last_run: u128,
    duration: Duration,
    #[derivative(Debug = "ignore")]
    action: Action,
    is_async: bool,
    is_regular: bool,
}

impl Task {
    pub fn new(duration: Duration, action: Action, is_async: bool, is_regular: bool) -> Task {
        Task {
            last_run: time_ms(),
            duration,
            action,
            is_async,
            is_regular,
        }
    }

    pub fn next_run(&self) -> u128 {
        self.last_run + self.duration.as_millis()
    }

    pub fn run(&mut self, pool: &ThreadPool) {
        if self.is_regular {
            self.last_run = time_ms();
        }

        if self.is_async {
            let action = self.action.clone();
            pool.execute(move || {
                action.as_ref()();
            });
        } else {
            (self.action)();
        }
    }
}

#[derive(Debug)]
pub struct RtTimer {
    descriptor: Option<u128>,
    long_term: bool,
    rt: Runtime,
}

impl RtTimer {
    pub fn new(rt: &Runtime, long_term: bool) -> RtTimer {
        RtTimer {
            descriptor: None,
            long_term,
            rt: rt.clone(),
        }
    }

    pub fn after<A>(&mut self, time: Duration, action: A)
        where
            A: Fn() + 'static + Send + Sync,
    {
        self.stop();
        self.descriptor =
            Some(
                self.rt
                    .create_task(Arc::new(Box::new(action)), time, self.long_term, false),
            );
    }

    pub fn stop(&self) {
        if let Some(descriptor) = self.descriptor {
            self.rt.remove_task(descriptor);
        }
    }
}

impl Drop for RtTimer {
    fn drop(&mut self) {
        self.stop();
    }
}

#[derive(Debug, Clone)]
pub struct Background {
    descriptor: Option<u128>,
    rt: Runtime,
}

impl Background {
    pub fn every<A>(rt: &Runtime, time: Duration, long_term: bool, action: A) -> Background
        where
            A: Fn() + 'static + Send + Sync {
        let descriptor =
            Some(rt.create_task(Arc::new(Box::new(action)), time, long_term, true));

        Background {
            descriptor,
            rt: rt.clone(),
        }
    }

    pub fn stop(&self) {
        if let Some(descriptor) = self.descriptor {
            self.rt.remove_task(descriptor);
        }
    }
}

impl Drop for Background {
    fn drop(&mut self) {
        self.stop();
    }
}

pub fn time_ms() -> u128 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs() as u128 * 1000 + d.subsec_millis() as u128)
        .unwrap_or(0)
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;
    use crate::runtime::{RtTimer, Runtime};

    #[test]
    fn test_timer() {
        let rt = Runtime::new(2);

        let thread = thread::current();
        let mut timer = RtTimer::new(&rt, false);
        timer.after(Duration::from_secs(1), move || {
            thread.unpark();
        });
        thread::park();

        let thread = thread::current();
        timer.stop();
        timer.after(Duration::from_secs(1), move || {
            thread.unpark();
        });
        thread::park();
    }
}
