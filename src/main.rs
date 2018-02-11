extern crate mqtt;
extern crate time;
extern crate threadpool;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

use std::sync::Arc;

mod transport;
mod controller;
mod handler;
mod configuration;

use configuration::SwitchConfiguration;
use handler::MessageHandler;
use controller::{DeviceHolder, Spot, Switch, Tap};
use handler::{SwitchHandler, Odin};
use transport::{Mqtt, Message};

fn main() {
    let spot_1 = Arc::new(Spot::new("spot_1"));
    let spot_2 = Arc::new(Spot::new("spot_2"));
    let spot_3 = Arc::new(Spot::new("spot_3"));
    let spot_4 = Arc::new(Spot::new("spot_4"));
    let spot_5 = Arc::new(Spot::new("spot_5"));

    let switch_1 = Arc::new(Switch::new("switch_1", vec!(spot_1.clone(), spot_2.clone())));
    let switch_2 = Arc::new(Switch::new("switch_2", vec!(spot_3.clone(), spot_4.clone())));
    let switch_3 = Arc::new(Switch::new("switch_3", vec!(spot_5.clone())));

    let cold_water = Arc::new(Tap::new("cold_water"));
    let hot_water = Arc::new(Tap::new("hot_water"));
    let reverse_water = Arc::new(Tap::new("reverse_water"));
    let leak_sensor = Arc::new(Switch::new("leak_sensor", vec!(cold_water.clone(), hot_water.clone(), reverse_water.clone())));
    let leak_sensor_config = Arc::new(SwitchConfiguration::new(vec!(leak_sensor.clone())));

    let switch_config = Arc::new(SwitchConfiguration::new(vec!(switch_1.clone(), switch_2.clone(), switch_3.clone())));
    let switch = Arc::new(SwitchHandler::new(switch_config.clone()));

    let odin = Arc::new(Odin {});

    Mqtt::new("localhost:1883", "odin")
        .subscribe("/+/switch/+/", move |(out, msg)| {
            let switch = switch.clone();
            switch.on_message(msg, out);
        })
        .subscribe("/+/leak_sensor/+/", move |(out, msg)| {
            odin.on_message(msg, out);
        })
        .run().unwrap();
}