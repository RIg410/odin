mod lighting;

pub use super::transport::{MqPublisher as Mqtt, TransportError};
use std::sync::{RwLock, Arc};
pub use controller::lighting::Lighting;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Device: Send + Sync + Debug {
    fn is_on(&self);
    fn is_off(&self);
    fn on(&self);
    fn off(&self);
    fn toggle(&self);
    fn set_state(&self);
    fn flush(&self, mqtt: &mut Mqtt) -> Result<(), TransportError>;
}

pub struct DeviceHolder {
    devices: HashMap<String, Box<Device>>
}

impl DeviceHolder {
    pub fn new() -> DeviceHolder {
        DeviceHolder { devices: HashMap::new() }
    }

    pub fn get(&self, id: &str) -> Option<&Box<Device>> {
        self.devices.get(id)
    }
}