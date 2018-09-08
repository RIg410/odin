extern crate serial;
extern crate dotenv;
extern crate mqtt;
extern crate time;
extern crate threadpool;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate bson;

use dotenv::dotenv;

use std::sync::Arc;
use std::env;

mod transport;
mod controller;
mod handler;
mod configuration;
mod serial_channel;

use serial_channel::SerialChannel;
use configuration::SwitchConfiguration;
use handler::MessageHandler;
use controller::{Spot, Switch, SerialDimmer, SerialSpot};
use handler::SwitchHandler;
use transport::Mqtt;
use transport::Message;

fn main() {
    dotenv().ok();
    let channel = SerialChannel::new();
    let bathroom_spot = SerialDimmer::new("bathroom", 0x01, channel.clone());
    let corridor_lamp = SerialDimmer::new("corridor_lamp", 0x03, channel.clone());
    let toilet_spot = SerialDimmer::new("toilet", 0x02, channel.clone());
    let kitchen_lamp = SerialDimmer::new("kitchen_lamp", 0x04, channel.clone());

    let bedroom_lamp = SerialSpot::new("bedroom_lamp", 0x01, channel.clone());
    let lounge_lamp = SerialSpot::new("lounge_lamp", 0x02, channel.clone());

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

    loop {
        let switch = switch.clone();
        let mq = env::var("MQTT").unwrap_or("localhost:1883".to_owned());
        println!("connect to MQTT: {}", mq);
        if let Err(err) = Mqtt::new(&mq, "odin")
            .subscribe("/switch/+", move |(out, msg)| {
                switch.on_message(msg, out);
            })
            .subscribe("/odin/ping", move |(out, msg)| {
                out.publish(Message::new("/odin/pong", msg.payload.to_owned()));
            })
            .run() {
            println!("Failed to start:{:?}", err);
        }

        std::thread::sleep_ms(2000);
    }
}