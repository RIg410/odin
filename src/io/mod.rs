use home::Home;
use io::serial::SerialChannel;
use io::web::WebChannel;
use sensors::{ActionType, Switch};
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};
use std::sync::Arc;

mod serial;
mod web;

use devices::Control;
pub use io::serial::Cmd;
use serde_json::Value;

pub trait Input {
    fn update_device(&self, name: &str, value: Value) -> Result<(), String>;
    fn act(&self, home: &Home, sensor_name: &str, action_type: ActionType) -> Result<(), String>;
    fn reg_web_devices(&self, ids: Vec<String>, host: String);
    fn devices_list(&self) -> Vec<String>;
    fn get_device(&self, name: &str) -> Result<Value, String>;
}

pub trait Output {
    fn serial_write(&self, cmd: Cmd);
    fn send(&self, id: &str, args: Vec<String>);
}

#[derive(Clone)]
pub struct IO {
    serial: SerialChannel,
    web: WebChannel,
    sensors: Option<Arc<SensorsHolder>>,
    devices: Option<Arc<DevicesHolder>>,
}

impl IO {
    pub fn create_mut() -> IOBuilder {
        let io = IO {
            serial: SerialChannel::new(),
            web: WebChannel::new(),
            sensors: None,
            devices: None,
        };
        IOBuilder {
            io,
            sensors: HashMap::new(),
            devices: HashMap::new(),
        }
    }
}

impl Output for IO {
    fn serial_write(&self, cmd: Cmd) {
        self.serial.send(cmd);
    }

    fn send(&self, id: &str, args: Vec<String>) {
        self.web.send(id, args)
    }
}

impl Input for IO {
    fn update_device(&self, name: &str, value: Value) -> Result<(), String> {
        if let Some(devices) = &self.devices {
            devices.update_device(name, value)
        } else {
            Err("IO is not initialized".to_owned())
        }
    }

    fn act(&self, home: &Home, sensor_name: &str, action_type: ActionType) -> Result<(), String> {
        if let Some(sensors) = &self.sensors {
            sensors.act(home, sensor_name, action_type)
        } else {
            Err("IO is not initialized".to_owned())
        }
    }

    fn reg_web_devices(&self, ids: Vec<String>, host: String) {
        self.web.reg_device(ids, host);
    }

    fn devices_list(&self) -> Vec<String> {
        self.devices
            .as_ref()
            .map(|d| d.devices.keys().map(ToOwned::to_owned).collect())
            .unwrap_or_default()
    }

    fn get_device(&self, name: &str) -> Result<Value, String> {
        if let Some(devices) = &self.devices {
            devices.get_device(name)
        } else {
            Err("IO is not initialized".to_owned())
        }
    }
}

impl Debug for IO {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Transport {{}}")
    }
}

pub struct IOBuilder {
    io: IO,
    sensors: HashMap<String, Switch>,
    devices: HashMap<String, Box<dyn Control>>,
}

impl IOBuilder {
    pub fn shared(&self) -> IO {
        self.io.clone()
    }

    pub fn build(self) -> IO {
        let IOBuilder {
            io,
            sensors,
            devices,
        } = self;
        let mut io = io;
        io.devices = Some(Arc::new(DevicesHolder { devices }));
        io.sensors = Some(Arc::new(SensorsHolder { sensors }));

        io
    }

    pub fn add_sensor(&mut self, switch: Switch) {
        self.sensors.insert(switch.id.as_str().to_owned(), switch);
    }

    pub fn reg_device(&mut self, device: Box<dyn Control>) {
        self.devices.insert(device.id().to_owned(), device);
    }
}

#[derive(Clone)]
pub struct SensorsHolder {
    sensors: HashMap<String, Switch>,
}

impl SensorsHolder {
    fn act(&self, home: &Home, sensor_name: &str, action_type: ActionType) -> Result<(), String> {
        if let Some(switch) = self.sensors.get(sensor_name) {
            switch.act(home, action_type)
        } else {
            Err(format!("Sensor with name '{}' not found.", sensor_name))
        }
    }
}

pub struct DevicesHolder {
    devices: HashMap<String, Box<dyn Control>>,
}

impl DevicesHolder {
    pub fn update_device(&self, name: &str, value: Value) -> Result<(), String> {
        self.devices
            .get(name)
            .ok_or_else(|| format!("device {} not found", name))
            .and_then(|dev| dev.update(value))
    }

    pub fn get_device(&self, name: &str) -> Result<Value, String> {
        self.devices
            .get(name)
            .ok_or_else(|| format!("device {} not found", name))
            .map(|dev| dev.load())
    }
}
