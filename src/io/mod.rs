use io::serial::{SerialChannel};
use std::fmt::{Debug, Formatter, Error};
use io::web::WebChannel;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use sensors::{Switch, OnSwitch};
use home::Home;

mod serial;
mod web;

pub use io::serial::Cmd;

#[derive(Clone)]
pub struct IO {
    serial: SerialChannel,
    web: WebChannel,
    sensors: SensorsHolder,
}

impl IO {
    pub fn create() -> IO {
        IO {
            serial: SerialChannel::new(),
            web: WebChannel::new(),
            sensors: SensorsHolder::new(),
        }
    }

    pub fn serial_write(&self, cmd: Cmd) {
        self.serial.send(cmd);
    }

    pub fn reg_sensor(&mut self, switch: Switch) -> Switch {
        self.sensors.add_sensor(switch.clone());
        switch
    }
}

impl Debug for IO {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Transport {{}}")
    }
}

#[derive(Clone)]
pub struct SensorsHolder {
    switch_map: Arc<HashMap<String, Switch>>
}

impl SensorsHolder {
    pub fn new() -> SensorsHolder {
        SensorsHolder {
            switch_map: Arc::new(HashMap::new())
        }
    }

    pub fn add_sensor(&mut self, switch: Switch) {
        if let Some(switch_map) = Arc::get_mut(&mut self.switch_map) {
            switch_map.insert(switch.id.as_str().to_owned(), switch);
        } else {
            println!("Failed to add sensor :{:?}", switch);
        }
    }

    pub fn on(&self, home: &Home, name: &str) {
        if let Some(switch) = self.switch_map.get(name) {
            switch.on(home);
        } else {
            println!("Sensor with name '{}' not found.", name);
        }
    }

    pub fn off(&self, home: &Home, name: &str) {
        if let Some(switch) = self.switch_map.get(name) {
            switch.off(home);
        } else {
            println!("Sensor with name '{}' not found.", name);
        }
    }

    pub fn toggle(&self, home: &Home, name: &str) {
        if let Some(switch) = self.switch_map.get(name) {
            switch.off(home);
        } else {
            println!("Sensor with name '{}' not found.", name);
        }
    }
}