extern crate serial as uart;
extern crate actix_web;
extern crate futures;
extern crate dotenv;
extern crate tokio_core;

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
use std::sync::Mutex;
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

    devices += SerialDimmer::new("bathroom_lamp", 0x01, serial_channel.clone(), true);//25-100
    devices += SerialDimmer::new("corridor_lamp", 0x03, serial_channel.clone(), true); //0-100
    devices += SerialDimmer::new("toilet_lamp", 0x02, serial_channel.clone(), true);//25-100
    devices += SerialDimmer::new("kitchen_lamp", 0x04, serial_channel.clone(), true);//0-100
    devices += SerialDimmer::new("bedroom_lamp", 0x01, serial_channel.clone(), false);
    devices += SerialDimmer::new("lounge_lamp", 0x02, serial_channel.clone(), false);
    devices += SerialDimmer::new("toilet_fun", 0x03, serial_channel.clone(), false);
    devices += SerialDimmer::new("bathroom_fun", 0x04, serial_channel.clone(), false);
    devices += SerialDimmer::new("device_5", 0x05, serial_channel.clone(), false);
    devices += SerialDimmer::new("lounge_cupboard_lamp", 0x06, serial_channel.clone(), false);
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

    let mut switch_list = vec![
        Switch::empty("corridor_2"),
        Switch::device("toilet", devices.dev("toilet_lamp")),
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

#[derive(Debug)]
struct IRState {
    front_door: bool,
    bedroom_door: bool,
    middle: bool,
    living_room: bool,
    corridor_lamp: bool,
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
    corridor_lamp: DeviceBox,
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
                })),
            corridor_lamp: devices.dev("corridor_lamp"),
        }

    }

    pub fn on_front_door(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.front_door = action_type == ActionType::On;
        self.handle_corridor_lamp(&mut state);
    }

    pub fn on_bedroom_door(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.bedroom_door = action_type == ActionType::On;
        self.handle_corridor_lamp(&mut state);
    }

    pub fn on_middle(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.middle = action_type == ActionType::On;
        self.handle_corridor_lamp(&mut state);
    }

    pub fn on_living_room(&self, action_type: ActionType) {
        let mut state = self.state.lock().unwrap();
        state.living_room = action_type == ActionType::On;
        self.handle_corridor_lamp(&mut state);
    }

    pub fn handle_corridor_lamp(&self, state: &mut IRState) {
        if state.is_some_on() && !state.corridor_lamp {
            state.corridor_lamp = true;

            return;
        }

        if state.is_all_off() && state.corridor_lamp {
            state.corridor_lamp = false;

            return;
        }
    }
}
