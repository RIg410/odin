mod lighting;
mod switch;
mod tap;

pub use super::transport::{TransportError as TError, Message};
use std::sync::PoisonError;
pub use controller::lighting::{MqttSpot, SerialDimmer, SerialSpot, Dimmer};
pub use controller::tap::Tap;
pub use controller::switch::Switch;
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Device: Send + Sync + Debug {
    fn is_on(&self) -> Result<bool, ControllerError>;
    fn is_off(&self) -> Result<bool, ControllerError>;
    fn on(&self) -> Result<(), ControllerError>;
    fn off(&self) -> Result<(), ControllerError>;
    fn toggle(&self) -> Result<bool, ControllerError>;
    fn flush(&self) -> Result<(), ControllerError>;
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

#[derive(Debug)]
pub enum ControllerError {
    GardError(String),
    TransportError(TError),
}

impl From<TError> for ControllerError {
    fn from(err: TError) -> ControllerError {
        ControllerError::TransportError(err)
    }
}

impl<T> From<PoisonError<T>> for ControllerError {
    fn from(err: PoisonError<T>) -> ControllerError {
        ControllerError::GardError(err.to_string())
    }
}