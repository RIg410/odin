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
mod sensors;
mod timer;
mod web;

use home::Home;
use io::IO;
use web::AppState;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "my_errors=warn,actix_web=warn");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv::dotenv().ok();
    let mut io = IO::create_mut();
    let home = Home::new(&mut io);
    info!("home: {:?}", home);
    let io = io.build();
    web::start_io(AppState::new(home, io)).await
}
