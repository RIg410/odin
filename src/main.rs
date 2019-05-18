extern crate serial as uart;
extern crate actix_web;
extern crate futures;
extern crate dotenv;
extern crate tokio_core;
extern crate chrono;

mod controller;
mod io;
mod timer;
mod web;
mod home;
mod devices;
mod sensors;

use io::IO;
use home::Home;
use web::AppState;
//
//use io::serial::SerialChannel;
//use controller::{SerialDimmer, WebDimmer, Switch, DeviceHandler, WebLed, ActionType, WebBeam};
//use io::web::WebChannel;
//use controller::Device;
//use controller::DeviceBox;

fn main() {
    dotenv::dotenv().ok();
    let mut io = IO::create_mut();
    let home = Home::new(&mut io);
    println!("home: {:?}", home);
    let io = io.build();
    web::start_io(AppState::new(home, io));
}

//fn init_devices(web_controller: &WebChannel) -> DeviceHandler {
//    let mut devices = DeviceHandler::new();
//    let serial_channel = SerialChannel::new();
//
//    devices += SerialDimmer::dimmer("bathroom_lamp", 0x01, &serial_channel, 20, 100);
//    devices += SerialDimmer::dimmer("corridor_lamp", 0x03, &serial_channel, 1, 100);
//    devices += SerialDimmer::dimmer("toilet_lamp", 0x02, &serial_channel, 25, 100);
//    devices += SerialDimmer::dimmer("kitchen_lamp", 0x04, &serial_channel, 1, 100);
//
//    devices += SerialDimmer::switch("bedroom_lamp", 0x01, &serial_channel);
//    devices += SerialDimmer::switch("lounge_lamp", 0x02, &serial_channel);
//    devices += SerialDimmer::switch("toilet_fun", 0x03, &serial_channel);
//    devices += SerialDimmer::switch("bathroom_fun", 0x04, &serial_channel);
//    devices += SerialDimmer::switch("device_5", 0x05, &serial_channel);
//    devices += SerialDimmer::switch("lounge_cupboard_lamp", 0x06, &serial_channel);
//
//    devices += WebDimmer::new("hot_water", &web_controller);
//    devices += WebDimmer::new("cold_water", &web_controller);
//    devices += WebDimmer::new("return_water", &web_controller);
//
//    devices += WebBeam::new("kitchen_beam", &web_controller);
//    devices += WebBeam::new("bedroom_beam_bed", &web_controller);
//    devices += WebBeam::new("bedroom_beam_table", &web_controller);
//    devices += WebBeam::new("corridor_beam", &web_controller);
//    devices += WebBeam::new("lounge_beam", &web_controller);
//
//    devices
//}

// return_water - это холодная вода
// cold_water <-> return_water
//fn init_switch(devices: DeviceHandler) -> SwitchHandler {
//    let exit_devices = devices.clone();
//    let corridor_lamp = devices.dev("corridor_lamp");
//    let corridor_beam_lamp = devices.dev("corridor_beam");
//
//    let toilet_lamp = devices.dev("toilet_lamp");
//    let toilet_fun = devices.dev("toilet_fun");
//
//    let corridor_lamp_toilet_copy = corridor_lamp.clone();
//    let mut switch_list = vec![
//        Switch::empty("corridor_2"),
//        Switch::lambda("toilet", move |a| {
//            let power = if corridor_lamp_toilet_copy.is_on() {
//                corridor_lamp_toilet_copy.power()
//            } else {
//                100
//            };
//
//            toilet_lamp.set_state(&a, power);
//
//            if a == ActionType::On {
//                toilet_fun.switch(&ActionType::Off);
//            } else {
//                toilet_fun.switch(&ActionType::On);
//                toilet_fun.delay(Duration::from_secs(60 * 5), |d| {
//                    d.switch(&ActionType::Off);
//                });
//            }
//        }),
//        Switch::device("lounge_cupboard_switch", devices.dev("lounge_cupboard_lamp")),
//        Switch::device("bathroom", devices.dev("bathroom_lamp")),
//        Switch::device("bedroom_1", devices.dev("bedroom_lamp")),
//        Switch::devices2("bedroom_2", devices.dev("bedroom_beam_bed"), devices.dev("bedroom_beam_table")),
//        Switch::device("lounge_1", devices.dev("lounge_lamp")),
//        Switch::device("lounge_2", devices.dev("lounge_beam")),
//        Switch::device("kitchen_1", devices.dev("kitchen_lamp")),
//        Switch::device("kitchen_2", devices.dev("kitchen_beam")),
//        Switch::empty("balcony_1"),
//        Switch::empty("balcony_2"),
//        Switch::devices3("water", devices.dev("hot_water"), devices.dev("cold_water"), devices.dev("return_water")),
//        Switch::lambda("exit_1", move |a| {
//            corridor_lamp.set_state(&a, 100);
//            corridor_beam_lamp.switch(&a);
//        }),
//        Switch::lambda("exit_2", move |_| {
//            exit_devices.for_each(|d| {
//                if d.id() == "hot_water" || d.id() == "cold_water" || d.id() == "return_water" {
//                    return;
//                }
//                if d.id() == "corridor_lamp" {
//                    d.set_state(&ActionType::On, 5);
//                    d.delay(Duration::from_secs(30), |d| {
//                        d.switch(&ActionType::Off);
//                        d.set_power(100);
//                    });
//                } else {
//                    d.switch(&ActionType::Off)
//                }
//            });
//        }),
//    ];
//
//    switch_list.append(&mut init_sensor_switch(devices.clone()));
//    SwitchHandler::new(switch_list)
//}

//fn init_sensor_switch(devices: DeviceHandler) -> Vec<Switch> {
//    let ir_front_door = IRHandler::new(&devices);
//    let ir_bedroom_door = ir_front_door.clone();
//    let ir_middle = ir_front_door.clone();
//    let ir_middle_1 = ir_front_door.clone();
//    let ir_living_room = ir_front_door.clone();
//    let ir_living_room_1 = ir_front_door.clone();
//    vec![
//        Switch::lambda("ir_sensor_front_door", move |t| ir_front_door.on_front_door(t)),
//        Switch::lambda("ir_sensor_bedroom_door", move |t| ir_bedroom_door.on_bedroom_door(t)),
//        Switch::lambda("ir_sensor_middle", move |t| ir_middle.on_middle(t)),
//        Switch::lambda("ir_sensor_middle_1", move |t| ir_middle_1.on_middle(t)),
//        Switch::lambda("ir_sensor_living_room", move |t| ir_living_room.on_living_room(t)),
//        Switch::lambda("ir_sensor_living_room_1", move |t| ir_living_room_1.on_living_room(t))
//    ]
//}

//#[derive(Debug)]
//struct IRState {
//    front_door: bool,
//    bedroom_door: bool,
//    middle: bool,
//    living_room: bool,
//    corridor_lamp: bool,
//    stamp: StampedSwitch,
//}
//
//impl IRState {
//    fn is_all_off(&self) -> bool {
//        !(self.front_door || self.bedroom_door || self.middle || self.living_room)
//    }
//
//    fn is_some_on(&self) -> bool {
//        self.front_door || self.bedroom_door || self.middle || self.living_room
//    }
//}
//
//#[derive(Clone, Debug)]
//struct IRHandler {
//    state: Arc<Mutex<IRState>>,
//}
//
//impl IRHandler {
//    pub fn new(devices: &DeviceHandler) -> IRHandler {
//        IRHandler {
//            state: Arc::new(Mutex::new(
//                IRState {
//                    front_door: false,
//                    bedroom_door: false,
//                    middle: false,
//                    living_room: false,
//                    corridor_lamp: false,
//                    stamp: StampedSwitch::new(devices.dev("corridor_lamp")),
//                })),
//        }
//    }
//
//    pub fn on_front_door(&self, action_type: ActionType) {
//        let mut state = self.state.lock().unwrap();
//        state.stamp.send(StampMessage::On(5 * 60 * 1000, SensorName::FrontDoor));
//    }
//
//    pub fn on_bedroom_door(&self, action_type: ActionType) {
//        let mut state = self.state.lock().unwrap();
//        state.stamp.send(StampMessage::On(2 * 60 * 1000, SensorName::BedroomDoor));
//    }
//
//    pub fn on_middle(&self, action_type: ActionType) {
//        let mut state = self.state.lock().unwrap();
//        state.stamp.send(StampMessage::On(2 * 60 * 1000, SensorName::Middle));
//    }
//
//    pub fn on_living_room(&self, action_type: ActionType) {
//        let mut state = self.state.lock().unwrap();
//        state.stamp.send(StampMessage::On(2 * 60 * 1000, SensorName::LivingRoom));
//    }
//}
//
//use std::sync::mpsc::channel;
//use std::time::SystemTime;
//use std::sync::mpsc::Sender;
//use std::thread::JoinHandle;
//use io::IO;
//use home::Home;
//
//#[derive(Debug)]
//struct StampedSwitch {
//    rx: Sender<StampMessage>,
//    thread: JoinHandle<()>,
//}
//
//impl StampedSwitch {
//    fn new(dev: DeviceBox) -> StampedSwitch {
//        let (rx, tx) = channel::<StampMessage>();
//
//        let thread = thread::spawn(move || {
//            let mut off_time = time();
//            loop {
//                if dev.is_on() {
//                    if let Ok(val) = tx.recv_timeout(Duration::from_secs(1)) {
//                        match val {
//                            StampMessage::On(offset, sensor) => {
//                                off_time = time() + offset;
//                            }
//                            StampMessage::Off => {
//                                // NO-op
//                            }
//                        }
//                    }
//
//                    if off_time < time() {
//                        dev.set_state(&ActionType::Off, 0);
//                    }
//                } else {
//                    if let Ok(val) = tx.recv() {
//                        match val {
//                            StampMessage::On(offset, sensor) => {
//                                dev.set_state(&ActionType::On, StampedSwitch::calc_power(sensor));
//                                off_time = time() + offset;
//                            }
//                            StampMessage::Off => {
//                                // NO-op
//                            }
//                        }
//                    }
//                }
//            }
//        });
//
//        StampedSwitch {
//            rx,
//            thread,
//        }
//    }
//
//    fn calc_power(sensor: SensorName) -> u8 {
//        if sensor == SensorName::FrontDoor {
//            100
//        } else {
//            let time = Local::now();
//            if time.hour() >= 22 || time.hour() <= 5 {
//                1
//            } else {
//                100
//            }
//        }
//    }
//
//    fn send(&self, msg: StampMessage) {
//        let time = Local::now();
//        if time.hour() > 17 || time.hour() < 10 {
//            self.rx.send(msg);
//        } else {
//            match msg {
//                StampMessage::On(_, SensorName::FrontDoor) => {
//                    self.rx.send(msg);
//                }
//                _ => {}
//            }
//        }
//    }
//}
//
//fn time() -> u128 {
//    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).ok()
//        .map(|d| d.as_secs() as u128 * 1000 + d.subsec_millis() as u128)
//        .unwrap_or(0)
//}
//
//
//#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
//enum SensorName {
//    LivingRoom,
//    Middle,
//    BedroomDoor,
//    FrontDoor,
//}
//
//enum StampMessage {
//    On(u128, SensorName),
//    Off,
//}
