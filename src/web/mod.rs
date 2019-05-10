use home::Home;
use std::sync::Arc;
use io::IO;
use std::collections::HashMap;

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

    pub fn update_device(&self, name: &str, state: HashMap<String, String>) -> Result<(), String> {
        self.io.devices.update_device(name, state)
    }
}

pub fn start_io(app_state: AppState) {
    web::run_web_service(app_state);
}