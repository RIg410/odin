use controller::SwitchHandler;
use controller::DeviceHandler;
use transport::web::WebChannel;
use std::thread;
use home::Home;
use std::sync::Arc;
use transport::Transport;

mod web;

#[derive(Clone)]
pub struct AppState {
    pub home: Arc<Home>,
    pub transport: Transport,
}

impl AppState {
    pub fn new(home: Home, transport: Transport) -> AppState {
        AppState {
            home: Arc::new(home),
            transport
        }
    }
}

pub fn start_io(app_state: AppState) {
    web::run_web_service(app_state);
}