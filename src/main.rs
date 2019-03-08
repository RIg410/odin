extern crate serial as uart;
extern crate actix_web;
extern crate futures;
extern crate dotenv;
extern crate tokio_core;
extern crate chrono;

mod controller;
mod serial;
mod web;
mod timer;
mod io;

use serial::SerialChannel;
use controller::{SerialDimmer, WebDimmer, Switch, SwitchHandler, DeviceHandler, WebLed, ActionType};
use web::WebController;
use controller::Device;
use std::time::Duration;
use std::sync::Arc;
use std::sync::{
    Mutex, RwLock
};
use controller::DeviceBox;
use io::AppState;
use std::thread;

fn main() {
    dotenv::dotenv().ok();
    let web_controller = WebController::new();
    let devices = init_devices(&web_controller);
    let switch_handler = init_switch(devices.clone());
    let app_state = AppState::new(switch_handler, devices, web_controller);
    io::start_io(app_state);
}

fn init_devices(web_controller: &WebController) -> DeviceHandler {
    let mut devices = DeviceHandler::new();
    let serial_channel = SerialChannel::new();

    devices += SerialDimmer::dimmer("bathroom_lamp", 0x01, serial_channel.clone(), 20, 100);
    devices += SerialDimmer::dimmer("corridor_lamp", 0x03, serial_channel.clone(), 1, 100);
    devices += SerialDimmer::dimmer("toilet_lamp", 0x02, serial_channel.clone(), 25, 100);
    devices += SerialDimmer::dimmer("kitchen_lamp", 0x04, serial_channel.clone(), 0, 100);

    devices += SerialDimmer::switch("bedroom_lamp", 0x01, serial_channel.clone());
    devices += SerialDimmer::switch("lounge_lamp", 0x02, serial_channel.clone());
    devices += SerialDimmer::switch("toilet_fun", 0x03, serial_channel.clone());
    devices += SerialDimmer::switch("bathroom_fun", 0x04, serial_channel.clone());
    devices += SerialDimmer::switch("device_5", 0x05, serial_channel.clone());
    devices += SerialDimmer::switch("lounge_cupboard_lamp", 0x06, serial_channel.clone());

    devices += WebDimmer::new("bedroom_beam_bed_lamp", web_controller.clone());
    devices += WebDimmer::new("bedroom_beam_table_lamp", web_controller.clone());
    devices += WebDimmer::new("corridor_beam_lamp", web_controller.clone());
    devices += WebDimmer::new("kitchen_beam_lamp", web_controller.clone());
    devices += WebDimmer::new("lounge_beam_bar_lamp", web_controller.clone());
    devices += WebDimmer::new("lounge_beam_main_lamp", web_controller.clone());
    devices += WebDimmer::new("hot_water", web_controller.clone());
    devices += WebDimmer::new("cold_water", web_controller.clone());
    devices += WebDimmer::new("return_water", web_controller.clone());
    devices += WebLed::new("bedroom_beam_led", web_controller.clone());
    devices += WebLed::new("corridor_beam_led", web_controller.clone());
    devices += WebLed::new("kitchen_beam_led", web_controller.clone());
    devices += WebLed::new("lounge_beam_bar_led", web_controller.clone());
    devices += WebLed::new("lounge_beam_main_led", web_controller.clone());

    devices
}
// return_water - это холодная вода
// cold_water <-> return_water

fn init_switch(devices: DeviceHandler) -> SwitchHandler {
    let exit_devices = devices.clone();
    let corridor_lamp = devices.dev("corridor_lamp");
    let corridor_beam_lamp = devices.dev("corridor_beam_lamp");

    let toilet_lamp = devices.dev("toilet_lamp");
    let toilet_fun = devices.dev("toilet_fun");

    let mut switch_list = vec![
        Switch::empty("corridor_2"),
        Switch::lambda("toilet", move |a| {
            toilet_lamp.switch(&a);

            if a == ActionType::On {
                toilet_fun.switch(&ActionType::Off);
            } else {
                toilet_fun.switch(&ActionType::On);
                toilet_fun.delay(Duration::from_secs(120), |d| {
                    d.switch(&ActionType::Off);
                });
            }
        }),
        Switch::device("lounge_cupboard_switch", devices.dev("lounge_cupboard_lamp")),
        Switch::device("bathroom", devices.dev("bathroom_lamp")),
        Switch::device("bedroom_1", devices.dev("bedroom_lamp")),
        Switch::devices2("bedroom_2", devices.dev("bedroom_beam_bed_lamp"), devices.dev("bedroom_beam_table_lamp")),
        Switch::device("lounge_1", devices.dev("lounge_lamp")),
        Switch::device("lounge_2", devices.dev("lounge_beam_main_lamp")),
        Switch::devices2("kitchen_1", devices.dev("kitchen_lamp"), devices.dev("lounge_beam_bar_lamp")),
        Switch::device("kitchen_2", devices.dev("kitchen_beam_lamp")),
        Switch::empty("balcony_1"),
        Switch::empty("balcony_2"),
        Switch::devices3("water", devices.dev("hot_water"), devices.dev("cold_water"), devices.dev("return_water")),
        Switch::lambda("exit_1", move |a| {
            corridor_lamp.set_state(&a, 100);
            corridor_beam_lamp.switch(&a);
        }),
        Switch::lambda("exit_2", move |_| {
            exit_devices.for_each(|d| {
                if d.id() == "hot_water" || d.id() == "cold_water" || d.id() == "return_water" {
                    return;
                }
                if d.id() == "corridor_lamp" {
                    d.set_state(&ActionType::On, 5);
                    d.delay(Duration::from_secs(30), |d| {
                        d.switch(&ActionType::Off);
                        d.set_power(100);
                    });
                } else {
                    d.switch(&ActionType::Off)
                }
            });
        }),
    ];

    switch_list.append(&mut init_sensor_switch(devices.clone()));
    SwitchHandler::new(switch_list)
}

fn init_sensor_switch(devices: DeviceHandler) -> Vec<Switch> {
    let ir_front_door = IRHandler::new(&devices);
    let ir_bedroom_door = ir_front_door.clone();
    let ir_middle = ir_front_door.clone();
    let ir_middle_1 = ir_front_door.clone();
    let ir_living_room = ir_front_door.clone();
    let ir_living_room_1 = ir_front_door.clone();
    vec![
        Switch::lambda("ir_sensor_front_door", move |t| ir_front_door.on_front_door(t)),//x3
        Switch::lambda("ir_sensor_bedroom_door", move |t| ir_bedroom_door.on_bedroom_door(t)),//x2
        Switch::lambda("ir_sensor_middle", move |t| ir_middle.on_middle(t)),//x2
        Switch::lambda("ir_sensor_middle_1", move |t| ir_middle_1.on_middle(t)),//x2
        Switch::lambda("ir_sensor_living_room", move |t| ir_living_room.on_living_room(t)), //x2;
        Switch::lambda("ir_sensor_living_room_1", move |t| ir_living_room_1.on_living_room(t)) //x2;
    ]
}

use chrono::prelude::*;

#[derive(Debug)]
struct IRState {
    front_door: bool,
    bedroom_door: bool,
    middle: bool,
    living_room: bool,
    corridor_lamp: bool,
    stamp: StampedSwitch,
}

impl IRState {
    fn is_all_off(&self) -> bool {
        !(self.front_door || self.bedroom_door || self.middle || self.living_room)
    }

    fn is_some_on(&self) -> bool {
        self.front_door || self.bedroom_door || self.middle || self.living_room
    }
}

#[derive(Clone, Debug)]
struct IRHandler {
    state: Arc<Mutex<IRState>>,
}

impl IRHandler {
    pub fn new(devices: &DeviceHandler) -> IRHandler {
        IRHandler {
            state: Arc::new(Mutex::new(
                IRState {
                    front_door: false,
                    bedroom_door: false,
                    middle: false,
                    living_room: false,
                    corridor_lamp: false,
                    stamp: StampedSwitch::new(devices.dev("corridor_lamp")),
                })),
        }
    }

    pub fn on_front_door(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.stamp.send(StampMessage::On(2 * 60 * 1000));
    }

    pub fn on_bedroom_door(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.stamp.send(StampMessage::On(2 * 60 * 1000));
    }

    pub fn on_middle(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.stamp.send(StampMessage::On(2 * 60 * 1000));
    }

    pub fn on_living_room(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.stamp.send(StampMessage::On(2 * 60 * 1000));
    }
}


use std::sync::mpsc::channel;
use std::time::SystemTime;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

#[derive(Debug)]
struct StampedSwitch {
    rx: Sender<StampMessage>,
    thread: JoinHandle<()>,
}

impl StampedSwitch {
    fn new(dev: DeviceBox) -> StampedSwitch {
        let (rx, tx) = channel::<StampMessage>();

        let thread = thread::spawn(move || {
            let mut off_time = time();
            loop {
                if dev.is_on() {
                    if let Ok(val) = tx.recv_timeout(Duration::from_secs(1)) {
                        match val {
                            StampMessage::On(offset) => {
                                off_time = time() + offset;
                            }
                            StampMessage::Off(_) => {
                                // NO-op
                            }
                        }
                    }

                    if off_time < time() {
                        dev.set_state(&ActionType::Off, 0);
                    }
                } else {
                    if let Ok(val) = tx.recv() {
                        match val {
                            StampMessage::On(offset) => {
                                dev.set_state(&ActionType::On, StampedSwitch::calc_power());
                                off_time = time() + offset;
                            }
                            StampMessage::Off(_) => {
                                // NO-op
                            }
                        }
                    }
                }
            }
        });

        StampedSwitch {
            rx,
            thread,
        }
    }

    fn calc_power() -> u8 {
        let time = Local::now();
        if time.hour() >= 22 || time.hour() <= 5 {
            return 1;
        }
        if time.hour() >= 21 {
            return 20;
        }
        if time.hour() >= 20 || time.hour() <= 7 {
            return 50;
        }

        return 100;
    }


    fn send(&self, msg: StampMessage) {
        let time = Local::now();
        if time.hour() > 17 || time.hour() < 10 {
            self.rx.send(msg);
        }
    }
}

fn time() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).ok()
        .map(|d| d.as_secs() as u128 * 1000 + d.subsec_millis() as u128)
        .unwrap_or(0)
}

enum StampMessage {
    On(u128),
    Off(u128),
}
