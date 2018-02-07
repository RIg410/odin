use super::Device;
use controller::{Mqtt, TransportError};
use std::sync::RwLock;

pub struct Lighting {}

#[derive(Debug)]
pub struct Spot {
    state: RwLock<SpotState>,
}

impl Spot {
    fn new() -> Spot {
        Spot { state: RwLock::new(SpotState::new()) }
    }
}

#[derive(Debug)]
struct SpotState {
    is_on: bool,
    brightness: u8,
}

impl SpotState {
    fn new() -> SpotState {
        SpotState { is_on: false, brightness: 40 }
    }
}

impl Device for Spot {
    fn is_on(&self) {
        self.state.re
        unimplemented!()
    }

    fn is_off(&self) {
        unimplemented!()
    }

    fn on(&self) {
        unimplemented!()
    }

    fn off(&self) {
        unimplemented!()
    }

    fn toggle(&self) {
        unimplemented!()
    }

    fn set_state(&self) {
        unimplemented!()
    }

    fn flush(&self, mqtt: &mut Mqtt) -> Result<(), TransportError> {
        unimplemented!()
    }
}
