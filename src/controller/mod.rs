mod lighting;
mod switch;

pub use controller::lighting::{SerialDimmer, WebDimmer, WebLed};
pub use controller::switch::{Switch, SwitchHandler};

use std::sync::PoisonError;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::str::FromStr;

pub trait Device: Send + Sync + Debug {
    fn id(&self) -> &str;
    fn is_on(&self) -> bool;
    fn power(&self) -> u8;
    fn switch(&self, action_type: &ActionType);
    fn set_power(&self, power: u8);
}

#[derive(PartialEq)]
pub enum ActionType {
    On,
    Off,
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "ON" => {
                Ok(ActionType::On)
            }
            "OFF" => {
                Ok(ActionType::Off)
            }
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum ControllerError {
    GardError(String),
}

impl<T> From<PoisonError<T>> for ControllerError {
    fn from(err: PoisonError<T>) -> ControllerError {
        ControllerError::GardError(err.to_string())
    }
}

#[derive(Clone)]
pub struct DeviceHandler {
    devices: Arc<RwLock<HashMap<String, Box<Device>>>>
}

impl DeviceHandler {
    pub fn new() -> DeviceHandler {
        DeviceHandler {
            devices: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn push<DEV>(&self, device: DEV) -> DEV
        where DEV: Device + Clone + 'static {
        let mut map = self.devices.write().unwrap();
        map.insert(device.id().to_string(), Box::new(device.clone()));
        device
    }

    pub fn switch(&self, id: &str, action_type: ActionType) {
        let map = self.devices.read().unwrap();
        if let Some(device) = map.get(id) {
            device.switch(&action_type)
        }
    }

    pub fn set_power(&self, id: &str, power: u8) {
        let map = self.devices.read().unwrap();
        if let Some(device) = map.get(id) {
            device.set_power(power)
        }
    }

    pub fn set_state(&self, id: &str, action_type: ActionType, power: u8) {
        let map = self.devices.read().unwrap();
        if let Some(device) = map.get(id) {
            device.set_power(power);
            device.switch(&action_type);
        }
    }
}
