use super::Device;
use controller::{Mqtt, Message, ControllerError};
use std::sync::Arc;
use std::sync::RwLock;
use serial_channel::{SerialChannel, Cmd};
use transport::MqPublisher;
use std::fmt;

const MIN: u8 = 1;
const MAX: u8 = 80;

#[derive(Debug)]
pub struct Spot {
    id: Arc<String>,
    state: Arc<RwLock<SpotState>>,
}

impl Spot {
    pub fn new(id: &str) -> Spot {
        Spot { id: Arc::new(id.to_owned()), state: Arc::new(RwLock::new(SpotState::new())) }
    }
}

impl Clone for Spot {
    fn clone(&self) -> Self {
        Spot { id: self.id.clone(), state: self.state.clone() }
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
        mqtt.publish(Message::new(&format!("/spot/{}", self.id), vec!(state.payload())))?;
        Ok(())
    }
}

pub struct SerialSpot {
    id: Arc<String>,
    p_id: u8,
    channel: SerialChannel,
    state: Arc<RwLock<SpotState>>,
}

impl Clone for SerialSpot {
    fn clone(&self) -> Self {
        SerialSpot { id: self.id.clone(), p_id: self.p_id.clone(), channel: self.channel.clone(), state: self.state.clone() }
    }
}

impl SerialSpot {
    pub fn new(id: &str, p_id: u8, channel: SerialChannel) -> SerialSpot {
        SerialSpot { id: Arc::new(id.to_owned()), p_id, channel, state: Arc::new(RwLock::new(SpotState::new())) }
    }
}

impl fmt::Debug for SerialSpot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SerialSpot {{ id: {}, p_id: {} state: {:?}}}", self.id, self.p_id, self.state)
    }
}

impl Device for SerialSpot {
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

    fn flush(&self, _: &mut MqPublisher) -> Result<(), ControllerError> {
        let state = self.state.read().unwrap();
        let arg = if state.is_on {
            if state.brightness == 0 {
                0x00
            } else {
                map(100 - state.brightness, 0, 100, 1, 80)
            }
        } else {
            0x00
        };

        self.channel.send(Cmd::new(0x01, self.p_id, arg));
        Ok(())
    }
}

fn map(x: u8, in_min: u8, in_max: u8, out_min: u8, out_max: u8) -> u8 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
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
