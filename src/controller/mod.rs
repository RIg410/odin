mod lighting;

pub use super::transport::MqPublisher as Mqtt;
use std::sync::{RwLock, Arc};
pub use controller::lighting::Lighting;
use std::collections::HashMap;

pub trait Device: Send + Sync {
    fn is_on(&self);
    fn is_off(&self);
    fn on(&self);
    fn off(&self);
    fn toggle(&self);
    fn set_state(&self);
    fn flush(&self, mqtt: &mut Mqtt);
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