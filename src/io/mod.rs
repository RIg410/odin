use crate::io::serial::SerialChannel;
use crate::io::web::WebChannel;
use crate::sensors::{ActionType, Switch};
use anyhow::{Error, Result};
use std::collections::HashMap;
use std::fmt::{Debug, Error as FmtError, Formatter};
use std::sync::Arc;

mod serial;
mod web;

use crate::devices::Control;
use crate::home::Home;
pub use crate::io::serial::Cmd;
use serde_json::Value;
use crate::timer::RT;

pub trait Input {
    fn update_device(&self, name: &str, value: Value) -> Result<()>;
    fn act(&self, home: &Home, sensor_name: &str, action_type: ActionType) -> Result<()>;
    fn reg_web_devices(&self, ids: Vec<String>, host: String);
    fn devices_list(&self) -> Vec<String>;
    fn get_device(&self, name: &str) -> Result<Value>;
}

pub trait Output {
    fn serial_write(&self, cmd: Cmd) -> Result<()>;
    fn send(&self, id: &str, args: Vec<String>) -> Result<()>;
}

#[derive(Clone)]
pub struct IO {
    serial: SerialChannel,
    web: WebChannel,
    sensors: Option<Arc<SensorsHolder>>,
    devices: Option<Arc<DevicesHolder>>,
    rt: RT,
}

impl IO {
    pub fn create_mut() -> IOBuilder {
        let io = IO {
            serial: SerialChannel::new(),
            web: WebChannel::new(),
            sensors: None,
            devices: None,
            rt: RT::new(2)
        };
        IOBuilder {
            io,
            sensors: HashMap::new(),
            devices: HashMap::new(),
        }
    }
}

impl Output for IO {
    fn serial_write(&self, cmd: Cmd) -> Result<()> {
        self.serial.send(cmd)
    }

    fn send(&self, id: &str, args: Vec<String>) -> Result<()> {
        self.web.send(id, args)
    }
}

impl Input for IO {
    fn update_device(&self, name: &str, value: Value) -> Result<()> {
        if let Some(devices) = &self.devices {
            devices.update_device(name, value)
        } else {
            Err(Error::msg("IO is not initialized"))
        }
    }

    fn act(&self, home: &Home, sensor_name: &str, action_type: ActionType) -> Result<()> {
        if let Some(sensors) = &self.sensors {
            sensors.act(home, sensor_name, action_type)
        } else {
            Err(Error::msg("IO is not initialized"))
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

    fn get_device(&self, name: &str) -> Result<Value> {
        if let Some(devices) = &self.devices {
            devices.get_device(name)
        } else {
            Err(Error::msg("IO is not initialized"))
        }
    }
}

impl Debug for IO {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
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

    pub fn rt(&self) -> &RT {
        &self.io.rt
    }
}

#[derive(Clone)]
pub struct SensorsHolder {
    sensors: HashMap<String, Switch>,
}

impl SensorsHolder {
    fn act(&self, home: &Home, sensor_name: &str, action_type: ActionType) -> Result<()> {
        if let Some(switch) = self.sensors.get(sensor_name) {
            switch.act(home, action_type)
        } else {
            Err(Error::msg(format!(
                "Sensor with name '{}' not found.",
                sensor_name
            )))
        }
    }
}

pub struct DevicesHolder {
    devices: HashMap<String, Box<dyn Control>>,
}

impl DevicesHolder {
    pub fn update_device(&self, name: &str, value: Value) -> Result<()> {
        self.devices
            .get(name)
            .ok_or_else(|| Error::msg(format!("device {} not found", name)))
            .and_then(|dev| dev.update(value))
    }

    pub fn get_device(&self, name: &str) -> Result<Value> {
        self.devices
            .get(name)
            .ok_or_else(|| Error::msg(format!("device {} not found", name)))
            .map(|dev| dev.load())
    }
}
