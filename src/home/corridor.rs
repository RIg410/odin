use crate::devices::{SerialDimmer, Switch as SwitchTrait, WebBeam};
use crate::home::Home;
use crate::io::IOMut;
use crate::runtime::time_ms;
use crate::sensors::Switch;
use anyhow::Result;
use chrono::{Local, Timelike};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use crate::home::scripts::{Runner, SWITCH_OFF_ALL};
use serde_json::Value;

#[derive(Debug)]
pub struct Corridor {
    pub lamp: SerialDimmer,
    pub beam: WebBeam,
    pub exit_1: Switch,
    pub exit_2: Switch,
    pub ir_sensor_front_door: Switch,
    pub ir_sensor_front_1_door: Switch,
    pub ir_sensor_bedroom_door: Switch,
    pub ir_sensor_middle: Switch,
    pub ir_sensor_middle_1: Switch,
    pub ir_sensor_living_room: Switch,
    pub ir_sensor_living_room_1: Switch,
    pub ir: IrHolder,
}

impl Corridor {
    pub fn new(io: &mut IOMut) -> Corridor {
        let ir_holder = IrHolder::new(Corridor::ir_handler);

        let ir_front_door = ir_holder.clone();
        let ir_front_door_1 = ir_holder.clone();
        let ir_bedroom_door = ir_holder.clone();
        let ir_middle = ir_holder.clone();
        let ir_middle_1 = ir_holder.clone();
        let ir_living_room = ir_holder.clone();
        let ir_living_room_1 = ir_holder.clone();

        Corridor {
            lamp: SerialDimmer::new(io, "corridor_lamp", 0x03, 1, 100),
            beam: WebBeam::new(io, "corridor_beam"),
            exit_1: Switch::new(io, "exit_1", Corridor::on_exit_1),
            exit_2: Switch::new(io, "exit_2", Corridor::on_exit_2),
            ir_sensor_front_door: Switch::new(io, "ir_sensor_front_door", move |home, is_on| {
                ir_front_door.ir_sensor_front_door(home, is_on)
            }),
            ir_sensor_front_1_door: Switch::new(
                io,
                "ir_sensor_front_1_door",
                move |home, is_on| ir_front_door_1.ir_sensor_front_1_door(home, is_on),
            ),
            ir_sensor_bedroom_door: Switch::new(
                io,
                "ir_sensor_bedroom_door",
                move |home, is_on| ir_bedroom_door.ir_sensor_bedroom_door(home, is_on), //todo ignore on light off
            ),
            ir_sensor_middle: Switch::new(io, "ir_sensor_middle", move |home, is_on| {
                ir_middle.ir_sensor_middle(home, is_on)
            }),
            ir_sensor_middle_1: Switch::new(io, "ir_sensor_middle_1", move |home, is_on| {
                ir_middle_1.ir_sensor_middle_1(home, is_on)
            }),
            ir_sensor_living_room: Switch::new(io, "ir_sensor_living_room", move |home, is_on| {
                ir_living_room.ir_sensor_living_room(home, is_on)
            }),
            ir_sensor_living_room_1: Switch::new(
                io,
                "ir_sensor_living_room_1",
                move |home, is_on| ir_living_room_1.ir_sensor_living_room_1(home, is_on),
            ),
            ir: ir_holder,
        }
    }

    pub fn enable_ir(&self) {
        self.ir.enable_ir();
    }

    pub fn disable_ir(&self) {
        self.ir.disable_ir();
    }

    fn on_exit_1(home: &Home, is_on: bool) -> Result<()> {
        home.corridor.beam.switch(is_on)
    }

    fn on_exit_2(home: &Home, _is_on: bool) -> Result<()> {
        home.run_script(SWITCH_OFF_ALL, Value::Null)
    }

    fn calc_power(_home: &Home, sensor: SensorName) -> u8 {
        if sensor == SensorName::FrontDoor {
            100
        } else {
            let time = Local::now();
            if time.hour() >= 22 || time.hour() <= 5 {
                3
            } else {
                100
            }
        }
    }

    fn ir_handler(home: &Home, is_on: bool, sensor_name: SensorName) -> Result<()> {
        if is_on {
            let power = Corridor::calc_power(home, sensor_name);
            home.corridor.lamp.set_power(power);
            home.corridor.lamp.switch(is_on)
        } else {
            home.corridor.lamp.switch(is_on)
        }
    }
}

#[derive(Debug)]
struct IrHandler {
    thread: JoinHandle<()>,
    tx: Sender<IrMessage>,
}

#[derive(Clone, Debug)]
pub struct IrHolder {
    handler: Arc<Mutex<IrHandler>>,
    is_ir_enable: Arc<AtomicBool>,
}

impl IrHolder {
    fn new<A>(act: A) -> IrHolder
    where
        A: Fn(&Home, bool, SensorName) -> Result<()> + Sync + Send + 'static,
    {
        let (tx, rx) = channel();
        let is_ir_enable = Arc::new(AtomicBool::new(true));
        let is_ir_enable_clone = is_ir_enable.clone();
        IrHolder {
            handler: Arc::new(Mutex::new(IrHandler {
                thread: thread::spawn(move || {
                    IrHolder::ir_loop(rx, is_ir_enable_clone, move |home, is_on, sensor_name| {
                        if let Err(err) = act(home, is_on, sensor_name) {
                            error!("Failed to handle ir action: {:?}", err);
                        }
                    })
                }),
                tx,
            })),
            is_ir_enable,
        }
    }

    pub fn enable_ir(&self) {
        self.is_ir_enable.store(true, Ordering::SeqCst);
    }

    pub fn disable_ir(&self) {
        self.is_ir_enable.store(false, Ordering::SeqCst);
    }

    fn ir_loop<A>(rx: Receiver<IrMessage>, is_ir_enable: Arc<AtomicBool>, act: A)
    where
        A: Fn(&Home, bool, SensorName) + Sync + Send + 'static,
    {
        let mut off_time = time_ms();
        let mut is_on = false;
        let mut home: Option<Home> = None;
        let mut sensor_name = SensorName::Middle;

        loop {
            if is_on {
                if let Ok(msg) = rx.recv_timeout(Duration::from_secs(1)) {
                    sensor_name = msg.sensor;
                    off_time = time_ms() + msg.duration as u128;
                    home = Some(msg.home);
                }

                if off_time < time_ms() {
                    is_on = false;
                    act(&home.as_ref().unwrap(), is_on, sensor_name.clone());
                }
            } else if let Ok(msg) = rx.recv() {
                if is_ir_enable.load(Ordering::SeqCst) {
                    sensor_name = msg.sensor;
                    is_on = true;
                    home = Some(msg.home);
                    off_time = time_ms() + msg.duration as u128;
                    act(&home.as_ref().unwrap(), is_on, sensor_name.clone());
                }
            }
        }
    }

    pub fn ir_sensor_front_door(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::FrontDoor);
        Ok(())
    }

    pub fn ir_sensor_front_1_door(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::FrontDoor);
        Ok(())
    }

    fn ir_sensor_bedroom_door(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::BedroomDoor);
        Ok(())
    }

    fn ir_sensor_middle(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::Middle);
        Ok(())
    }

    fn ir_sensor_middle_1(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::Middle);
        Ok(())
    }

    fn ir_sensor_living_room(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::LivingRoom);
        Ok(())
    }

    fn ir_sensor_living_room_1(&self, home: &Home, is_on: bool) -> Result<()> {
        self.send_msg(home, is_on, SensorName::LivingRoom);
        Ok(())
    }

    fn calc_duration(&self, sensor: &SensorName) -> u128 {
        if sensor == &SensorName::FrontDoor {
            Duration::from_secs(5 * 60).as_millis()
        } else {
            Duration::from_secs(2 * 60).as_millis()
        }
    }

    fn send_msg(&self, home: &Home, _is_on: bool, sensor: SensorName) {
        let time = Local::now();
        if time.hour() > 16 || time.hour() < 10 || sensor == SensorName::FrontDoor {
            self.handler
                .lock()
                .unwrap()
                .tx
                .send(IrMessage {
                    duration: self.calc_duration(&sensor),
                    sensor,
                    home: home.clone(),
                })
                .unwrap();
        }
    }
}

struct IrMessage {
    duration: u128,
    sensor: SensorName,
    home: Home,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Clone)]
enum SensorName {
    LivingRoom,
    Middle,
    BedroomDoor,
    FrontDoor,
}
