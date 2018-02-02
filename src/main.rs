extern crate mqtt;
extern crate time;
extern crate threadpool;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate chrono;

use std::sync::Arc;

mod transport;
mod controller;
mod handler;
mod configuration;

use configuration::SwitchConfiguration;
use handler::MessageHandler;
use controller::Lighting;
use handler::{Switch, Odin};
use transport::{Mqtt, Message};

fn main() {
    let switch_config = Arc::new(SwitchConfiguration::new());
    let lighting = Arc::new(Lighting {});
    let switch = Arc::new(Switch::new(lighting.clone(), switch_config.clone()));
    let odin = Arc::new(Odin {});

    Mqtt::new("localhost:1883", "odin")
        .subscribe("/+/switch/+/", move |(out, msg)| {
            let switch = switch.clone();
            switch.on_message(msg, out);
        })
        .subscribe("/freya/update/odin/", move |(out, msg)| {
            odin.on_message(msg, out);
        })
        .run().unwrap();
}