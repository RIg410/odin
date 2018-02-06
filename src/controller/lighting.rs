use super::Device;
use controller::Mqtt;
use std::sync::RwLock;

pub struct Lighting {}

pub struct Spot {
    state: RwLock<SpotState>,
}

impl Spot {
    fn new() -> Spot {
        Spot { state: RwLock::new(SpotState::new()) }
    }
}

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

    fn flush(&self, mqtt: &mut Mqtt) {
        unimplemented!()
    }
}
