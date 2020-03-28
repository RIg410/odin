use crate::home::{Home, BackgroundProcess};
use crate::io::{Input, IO};
use anyhow::Result;
use serde_json::Value;
use std::sync::Arc;

mod backend;

#[derive(Clone)]
pub struct AppState {
    pub home: Arc<Home>,
    pub io: IO,
    bg: BackgroundProcess,
}

impl AppState {
    pub fn new(home: Home, io: IO, bg: BackgroundProcess) -> AppState {
        AppState {
            home: Arc::new(home),
            io,
            bg,
        }
    }

    pub fn update_device(&self, name: &str, state: Value) -> Result<()> {
        self.io.update_device(name, state)
    }

    pub fn devices_list(&self) -> Vec<String> {
        self.io.devices_list()
    }

    pub fn get_device(&self, name: &str) -> Result<Value> {
        self.io.get_device(name)
    }
}

pub async fn start_io(app_state: AppState) -> std::io::Result<()> {
    backend::run_web_service(app_state).await
}
