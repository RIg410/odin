//use controller::DeviceHandler;
//use io::web::WebChannel;
use std::thread;
use home::Home;
use std::sync::Arc;
use io::IO;

mod web;

#[derive(Clone)]
pub struct AppState {
    pub home: Arc<Home>,
    pub io: IO,
}

impl AppState {
    pub fn new(home: Home, io: IO) -> AppState {
        AppState {
            home: Arc::new(home),
            io
        }
    }
}

pub fn start_io(app_state: AppState) {
    web::run_web_service(app_state);
}