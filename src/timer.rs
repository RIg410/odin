//use std::time::Duration;
//use std::{
//    thread,
//    thread::JoinHandle,
//    sync::atomic::AtomicBool,
//};
//use std::time::Instant;
//use std::sync::atomic::Ordering;
//use std::sync::Arc;
//
//#[derive(Debug)]
//pub struct Timer {
//    thread: Option<JoinHandle<()>>,
//    active: Option<Arc<AtomicBool>>,
//}
//
//impl Timer {
//    pub fn new() -> Timer {
//        Timer { thread: None, active: None }
//    }
//
//    pub fn after<A>(&mut self, time: Duration, action: A)
//        where A: Fn() + 'static + Send + Sync {
//        self.reset();
//
//        let run = Arc::new(AtomicBool::new(true));
//        self.active = Some(run.clone());
//        self.thread = Some(thread::spawn(move || {
//            let start = Instant::now();
//            loop {
//                thread::sleep(Duration::from_millis(100));
//                if start.elapsed() < time {
//                    continue;
//                }
//
//                if !run.load(Ordering::SeqCst) {
//                    return;
//                }
//                action();
//                return;
//            }
//        }));
//    }
//
//    pub fn reset(&mut self) {
//        match &self.active {
//            Some(run) => run.store(false, Ordering::SeqCst),
//            None => {}
//        }
//        self.thread = None;
//    }
//}
