use std::{
    thread,
    time::{Instant, Duration},
    thread::JoinHandle,
    sync::{
        Arc,
        atomic::AtomicBool,
        atomic::Ordering,
    },
};
use std::time::SystemTime;

#[derive(Debug)]
pub struct Timer {
    thread: Option<JoinHandle<()>>,
    active: Option<Arc<AtomicBool>>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer { thread: None, active: None }
    }

    pub fn after<A>(&mut self, time: Duration, action: A)
        where A: Fn() + 'static + Send + Sync {
        self.reset();

        let run = Arc::new(AtomicBool::new(true));
        self.active = Some(run.clone());
        self.thread = Some(thread::spawn(move || {
            let start = Instant::now();
            loop {
                thread::sleep(Duration::from_millis(100));
                if start.elapsed() < time {
                    continue;
                }

                if !run.load(Ordering::SeqCst) {
                    return;
                }
                action();
                return;
            }
        }));
    }

    pub fn reset(&mut self) {
        match &self.active {
            Some(run) => run.store(false, Ordering::SeqCst),
            None => {}
        }
        self.thread = None;
    }
}

pub fn time_ms() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).ok()
        .map(|d| d.as_secs() as u128 * 1000 + d.subsec_millis() as u128)
        .unwrap_or(0)
}