extern crate mqtt;
extern crate time;
extern crate threadpool;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate bson;
extern crate mongodb;

use std::sync::Arc;

mod transport;
mod controller;
mod handler;
mod configuration;

use configuration::SwitchConfiguration;
use handler::MessageHandler;
use controller::{Spot, Switch};
use handler::SwitchHandler;
use transport::Mqtt;

fn main() {
    let corridor_lamp = Spot::new("corridor_lamp");
    let toilet_spot = Spot::new("toilet");
    let bathroom_spot = Spot::new("bathroom");
    let bedroom_lamp = Spot::new("bedroom_lamp");
    let lounge_lamp = Spot::new("lounge_lamp");
    let kitchen_lamp = Spot::new("kitchen_lamp");

    let switch_list = vec![
        Switch::new("corridor_1", vec![Box::new(corridor_lamp.clone())]),
        Switch::new("corridor_2", vec![]),
        Switch::new("toilet", vec![Box::new(toilet_spot.clone())]),
        Switch::new("bathroom", vec![Box::new(bathroom_spot.clone())]),
        Switch::new("bedroom_1", vec![Box::new(bedroom_lamp.clone())]),
        Switch::new("bedroom_2", vec![]),
        Switch::new("lounge_1", vec![Box::new(lounge_lamp.clone())]),
        Switch::new("lounge_2", vec![]),
        Switch::new("kitchen_1", vec![Box::new(kitchen_lamp.clone())]),
        Switch::new("kitchen_2", vec![]),
        Switch::new("balcony_1", vec![]),
        Switch::new("balcony_2", vec![])
    ];

    let switch_config = Arc::new(SwitchConfiguration::new(switch_list));
    let switch = Arc::new(SwitchHandler::new(switch_config.clone()));

    Mqtt::new("192.168.1.137:1883", "odin")
        .subscribe("/switch/+", move |(out, msg)| {
            let switch = switch.clone();
            switch.on_message(msg, out);
        })
        .run().unwrap();
}