use home::Home;
use io::{Input, IO};
use serde_json::Value;
use std::sync::Arc;

mod backend;

#[derive(Clone)]
pub struct AppState {
    pub home: Arc<Home>,
    pub io: IO,
}

impl AppState {
    pub fn new(home: Home, io: IO) -> AppState {
        AppState {
            home: Arc::new(home),
            io,
        }
    }

    pub fn update_device(&self, name: &str, state: Value) -> Result<(), String> {
        self.io.update_device(name, state)
    }

    pub fn devices_list(&self) -> Vec<String> {
        self.io.devices_list()
    }

    pub fn get_device(&self, name: &str) -> Result<Value, String> {
        self.io.get_device(name)
    }
}

pub fn start_io(app_state: AppState) {
    backend::run_web_service(app_state);
}
