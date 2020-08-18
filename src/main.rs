extern crate actix_web;
extern crate chrono;
extern crate dotenv;
extern crate futures;
extern crate serial as uart;
extern crate tokio_core;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate anyhow;
extern crate serde;
#[macro_use]
extern crate log;

mod devices;
mod home;
mod io;
mod runtime;
mod sensors;
mod utils;
mod web;

use crate::home::configuration::Configuration;
use crate::home::BackgroundProcess;
use crate::runtime::Runtime;
use home::Home;
use io::IO;
use sentry::integrations::log::LoggerOptions;
use sentry::integrations::{env_logger::init, panic::register_panic_handler};
use sentry::{capture_message, Level};
use std::env;
use web::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let _guard = if let Ok(sentry_path) = env::var("SENTRY") {
        capture_message("Start home controller.", Level::Info);
        let guard = sentry::init(sentry_path);
        register_panic_handler();
        init(None, LoggerOptions::default());
        Some(guard)
    } else {
        env_logger::init();
        None
    };

    let config = Configuration::new(&env::var("DB").unwrap())?;
    let runtime = Runtime::new(2);
    let mut io = IO::with_runtime(&runtime);
    let home = Home::new(&mut io, &config);
    info!("home: {:?}", home);
    let io = io.freeze();
    let bg = BackgroundProcess::new(&home, &io, &config).unwrap();
    web::start_io(AppState::new(home, io, bg, config)).await
}
