use home::Home;
use std::sync::Arc;
use io::{IO, Input};
use serde_json::Value;

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
    web::run_web_service(app_state);
}