extern crate serial as uart;
extern crate actix_web;
extern crate futures;
extern crate dotenv;
extern crate tokio_core;
extern crate chrono;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod io;
mod timer;
mod web;
mod home;
mod devices;
mod sensors;

use io::IO;
use home::Home;
use web::AppState;

fn main() {
    std::env::set_var("RUST_LOG", "my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv::dotenv().ok();
    let mut io = IO::create_mut();
    let home = Home::new(&mut io);
    println!("home: {:?}", home);
    let io = io.build();
    web::start_io(AppState::new(home, io));
}
