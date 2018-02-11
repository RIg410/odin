use super::Device;
use controller::{Mqtt, TError, Message, ControllerError};
use std::sync::RwLock;

#[derive(Debug)]
pub struct Spot {
    id: String,
    state: RwLock<SpotState>,
}

impl Spot {
    pub fn new(id: &str) -> Spot {
        Spot { id: id.to_owned(), state: RwLock::new(SpotState::new()) }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct SpotState {
    is_on: bool,
    brightness: u8,
}

impl SpotState {
    fn new() -> SpotState {
        SpotState { is_on: false, brightness: 40 }
    }

    fn from_byte(byte: u8) -> SpotState {
        let is_on = byte & 0b10000000u8 == 0b10000000u8;
        let brightness = byte & 0b01111111u8;
        SpotState { is_on, brightness }
    }

    fn payload(&self) -> u8 {
        if self.is_on {
            self.brightness | 0b10000000u8
        } else {
            self.brightness & 0b01111111u8
        }
    }
}

impl Device for Spot {
    fn is_on(&self) -> Result<bool, ControllerError> {
        let state = self.state.read().unwrap();
        Ok(state.is_on)
    }

    fn is_off(&self) -> Result<bool, ControllerError> {
        self.is_on().map(|st| { !st })
    }

    fn on(&self) -> Result<(), ControllerError> {
        let mut state = self.state.write().unwrap();
        state.is_on = true;
        Ok(())
    }

    fn off(&self) -> Result<(), ControllerError> {
        let mut state = self.state.write().unwrap();
        state.is_on = false;
        Ok(())
    }

    fn toggle(&self) -> Result<bool, ControllerError> {
        let mut state = self.state.write().unwrap();
        state.is_on = !state.is_on;
        Ok(state.is_on)
    }

    fn flush(&self, mqtt: &mut Mqtt) -> Result<(), ControllerError> {
        let state = self.state.read().unwrap();
        mqtt.publish(Message::new(&format!("/odin/spot/{}/", self.id), vec!(state.payload())))?;
        Ok(())
    }
}

#[test]
fn test_spot_state() {
    let spot = SpotState { is_on: false, brightness: 40 };
    assert_eq!(spot, SpotState::from_byte(spot.payload()));

    let spot = SpotState { is_on: true, brightness: 100 };
    assert_eq!(spot, SpotState::from_byte(spot.payload()));

    let spot = SpotState { is_on: true, brightness: 0 };
    assert_eq!(spot, SpotState::from_byte(spot.payload()));
}
