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
extern crate actix_web;

use dotenv::dotenv;

use std::sync::Arc;
use std::env;

mod transport;
mod controller;
mod handler;
mod serial_channel;

use serial_channel::SerialChannel;
use controller::{Switch, SerialDimmer, SerialSpot};
use handler::SwitchHandler;
use transport::Mqtt;
use transport::{Message, MqttChannel};
use handler::parse_id;
use controller::Device;
use handler::SwitchHolder;
use std::thread;
use actix_web::server;
use actix_web::App;
use actix_web::http;
use actix_web::Path;
use actix_web::State;
use actix_web::Result as WebResult;
use handler::Dimmer;

fn main() {
    dotenv().ok();

    let mqtt_channel = MqttChannel::new();
    let serial_channel = SerialChannel::new();
    let bathroom_spot = SerialDimmer::new("bathroom", 0x01, serial_channel.clone());
    let corridor_lamp = SerialDimmer::new("corridor_lamp", 0x03, serial_channel.clone());
    let toilet_spot = SerialDimmer::new("toilet", 0x02, serial_channel.clone());
    let kitchen_lamp = SerialDimmer::new("kitchen_lamp", 0x04, serial_channel.clone());

    let bedroom_lamp = SerialSpot::new("bedroom_lamp", 0x01, serial_channel.clone());
    let lounge_lamp = SerialSpot::new("lounge_lamp", 0x02, serial_channel.clone());

    let mut dimmer = Dimmer::new();
    dimmer.add_serial_dimmer(corridor_lamp.clone());
    dimmer.add_serial_dimmer(bathroom_spot.clone());
    dimmer.add_serial_dimmer(toilet_spot.clone());
    dimmer.add_serial_dimmer(kitchen_lamp.clone());

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

    let switch_config = Arc::new(SwitchHolder::new(switch_list));
    let switch = Arc::new(SwitchHandler::new(switch_config.clone()));

    let dimmer = Arc::new(dimmer);
    let web_switch = switch.clone();
    thread::spawn(move || {
        server::new(move || {
            App::with_state(AppState { switch: web_switch.clone() })
                .prefix("/odin/api")
                .resource("switch/{switch}/{state}", |r| r.method(http::Method::GET).with(switch_hndl))
        })
            .bind("0.0.0.0:1884")
            .expect("Can not bind to port 8000")
            .run();
    });

    loop {
        let dimmer = dimmer.clone();
        let switch = switch.clone();
        let mqtt_channel = mqtt_channel.clone();
        let ping_channel = mqtt_channel.clone();
        let mq = env::var("MQTT").unwrap_or("localhost:1883".to_owned());
        println!("connect to MQTT: {}", mq);
        if let Err(err) = Mqtt::new(&mq, "odin")
            .subscribe("/switch/+", move |msg| {
                if let Some(id) = parse_id(msg.topic) {
                    switch.handle(id, String::from_utf8_lossy(&msg.payload).as_ref());
                } else {
                    println!("Failed to get switch id");
                }
            })
            .subscribe("/dimm/+", move |msg| {
                if let Some(id) = parse_id(msg.topic) {
                    if let Ok(payload) = msg.payload_as_string() {
                        if let Ok(d) = payload.parse::<u8>() {
                            dimmer.dimm(id, d)
                        }
                    }
                }
            })
            .subscribe("/odin/ping", move |msg| {
                ping_channel.publish(Message::new("/odin/pong", msg.payload.to_owned()));
            })
            .run(mqtt_channel.clone()) {
            println!("Failed to start:{:?}", err);
        }
        std::thread::sleep_ms(2000);
    }
}

pub struct AppState {
    pub switch: Arc<SwitchHandler>,
}

fn switch_hndl((params, state): (Path<(String, String)>, State<AppState>)) -> WebResult<String> {
    println!("switch:{}, state:{}", &params.0, &params.1);
    state.switch.handle(&params.0, &params.1);
    Ok("Ok".to_owned())
}
