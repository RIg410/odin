use io::serial::SerialChannel;
use std::fmt::{Debug, Formatter, Error};
use io::web::WebChannel;
use std::collections::HashMap;
use std::sync::Arc;
use sensors::{Switch, OnSwitch};
use home::Home;

mod serial;
mod web;

pub use io::serial::Cmd;
use devices::Update;

#[derive(Clone)]
pub struct IO {
    serial: SerialChannel,
    web: WebChannel,
    pub sensors: SensorsHolder,
    pub devices: DevicesHolder,
}

impl IO {
    pub fn create() -> IO {
        IO {
            serial: SerialChannel::new(),
            web: WebChannel::new(),
            sensors: SensorsHolder::new(),
            devices: DevicesHolder::new(),
        }
    }

    pub fn serial_write(&self, cmd: Cmd) {
        self.serial.send(cmd);
    }

    pub fn reg_sensor(&mut self, switch: Switch) -> Switch {
        self.sensors.add_sensor(switch.clone());
        switch
    }

    pub fn reg_device(&mut self, device: Box<Update>) {
        self.devices.add_device(device);
    }

    pub fn reg_web_devices(&self, ids: Vec<String>, host: String) {
        self.web.reg_device(ids, host);
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

    fn add_sensor(&mut self, switch: Switch) {
        if let Some(switch_map) = Arc::get_mut(&mut self.switch_map) {
            switch_map.insert(switch.id.as_str().to_owned(), switch);
        } else {
            panic!("Failed to add sensor :{:?}", switch);
        }
    }

    //TODO return result
    pub fn on(&self, home: &Home, name: &str) {
        if let Some(switch) = self.switch_map.get(name) {
            switch.on(home);
        } else {
            println!("Sensor with name '{}' not found.", name);
        }
    }

    //TODO return result
    pub fn off(&self, home: &Home, name: &str) {
        if let Some(switch) = self.switch_map.get(name) {
            switch.off(home);
        } else {
            println!("Sensor with name '{}' not found.", name);
        }
    }
    //TODO return result
    pub fn toggle(&self, home: &Home, name: &str) {
        if let Some(switch) = self.switch_map.get(name) {
            switch.off(home);
        } else {
            println!("Sensor with name '{}' not found.", name);
        }
    }
}

#[derive(Clone)]
pub struct DevicesHolder {
    devices_map: Arc<HashMap<String, Box<Update>>>
}

impl DevicesHolder {
    pub fn new() -> DevicesHolder {
        DevicesHolder {
            devices_map: Arc::new(HashMap::new())
        }
    }

    fn add_device(&mut self, device: Box<Update>) {
        if let Some(devices_map) = Arc::get_mut(&mut self.devices_map) {
            devices_map.insert(device.id().to_owned(), device);
        } else {
            panic!("Failed to add device :{:?}", device);
        }
    }

    pub fn update_device(&self, name: &str, value: HashMap<String, String>) -> Result<(), String> {
        self.devices_map.get(name)
            .ok_or(format!("device {} not found", name))
            .and_then(|dev| dev.update(value))
    }
}