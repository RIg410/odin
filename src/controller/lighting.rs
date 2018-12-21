use super::Device;
use std::sync::Arc;
use std::sync::RwLock;
use serial_channel::{SerialChannel, Cmd};
use std::fmt;
use controller::ActionType;
use web::WebController;

#[derive(Debug, Eq, PartialEq)]
struct SpotState {
    is_on: bool,
    brightness: u8,
}

impl SpotState {
    fn new() -> SpotState {
        SpotState { is_on: false, brightness: 100 }
    }
}

pub struct SerialDimmer {
    pub id: Arc<String>,
    p_id: u8,
    channel: SerialChannel,
    state: Arc<RwLock<SpotState>>,
    can_dim: bool,
}

impl Clone for SerialDimmer {
    fn clone(&self) -> Self {
        SerialDimmer { id: self.id.clone(), p_id: self.p_id.clone(), channel: self.channel.clone(), state: self.state.clone(), can_dim: self.can_dim.clone() }
    }
}

impl SerialDimmer {
    pub fn new(id: &str, p_id: u8, channel: SerialChannel, can_dim: bool) -> SerialDimmer {
        SerialDimmer { id: Arc::new(id.to_owned()), p_id, channel, state: Arc::new(RwLock::new(SpotState::new())), can_dim }
    }

    fn flush(&self) {
        let state = self.state.read().unwrap();
        if self.can_dim {
            let arg = if state.is_on {
                invert_and_map(state.brightness)
            } else {
                255
            };

            self.channel.send(Cmd::new(0x01, self.p_id, arg));
        } else {
            let arg = if state.is_on {
                0x01
            } else {
                0x02
            };
            self.channel.send(Cmd::new(0x02, self.p_id, arg));
        }
    }
}

impl Device for SerialDimmer {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_on(&self) -> bool {
        let state = self.state.read().unwrap();
        state.is_on
    }

    fn power(&self) -> u8 {
        let state = self.state.read().unwrap();
        state.brightness
    }

    fn switch(&self, action_type: &ActionType) {
        let mut state = self.state.write().unwrap();
        state.is_on = action_type == &ActionType::On;

        self.flush()
    }

    fn set_power(&self, dim: u8) {
        let mut state = self.state.write().unwrap();
        state.brightness = dim;

        self.flush()
    }
}

impl fmt::Debug for SerialDimmer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SerialDimmer {{ id: {}, p_id: {} state: {:?}}}", self.id, self.p_id, self.state)
    }
}

pub struct WebDimmer {
    pub id: Arc<String>,
    state: Arc<RwLock<SpotState>>,
    web: WebController
}

impl WebDimmer {
    pub fn new(id: &str, web: WebController) -> WebDimmer {
        WebDimmer {
            id: Arc::new(id.to_owned()),
            state: Arc::new(RwLock::new(SpotState::new())),
            web
        }
    }
}

impl Device for WebDimmer {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_on(&self) -> bool {
        unimplemented!()
    }

    fn power(&self) -> u8 {
        unimplemented!()
    }

    fn switch(&self, action_type: &ActionType) {
        unimplemented!()
    }

    fn set_power(&self, power: u8) {
        unimplemented!()
    }
}

impl fmt::Debug for WebDimmer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WebDimmer {{ id: {}, state: {:?}}}", self.id, self.state)
    }
}

impl Clone for WebDimmer {
    fn clone(&self) -> Self {
        WebDimmer { id: self.id.clone(), state: self.state.clone(), web: self.web.clone() }
    }
}

pub type Color = (u8, u8, u8);
pub type SpeedAndBrightness = (u8, u8);

#[derive(Debug)]
pub enum LedMode {
    Color(Color),
    Rainbow(SpeedAndBrightness),
    Borealis(SpeedAndBrightness),
}

#[derive(Debug)]
pub struct LedState {
    on: bool,
    mode: LedMode,
}
#[derive(Debug, Clone)]
pub struct WebLed {
    pub id: Arc<String>,
    state: Arc<RwLock<LedState>>,
    web: WebController
}

impl WebLed {
    pub fn new(id: &str, web: WebController) -> WebLed {
        WebLed {
            id: Arc::new(id.to_owned()),
            state: Arc::new(RwLock::new(LedState { on: false, mode: LedMode::Color((200, 200, 200)) })),
            web
        }
    }
}

impl Device for WebLed {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_on(&self) -> bool {
        unimplemented!()
    }

    fn power(&self) -> u8 {
        unimplemented!()
    }

    fn switch(&self, action_type: &ActionType) {
        unimplemented!()
    }

    fn set_power(&self, power: u8) {
        unimplemented!()
    }
}


#[inline]
fn map(x: u32, in_min: u32, in_max: u32, out_min: u32, out_max: u32) -> u32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

#[inline]
fn invert_and_map(val: u8) -> u8 {
    if val == 0 {
        255
    } else {
        map(100 - val as u32, 0, 100, 26, 229) as u8
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

#[test]
fn test_dimmer() {
    assert_eq!(255, invert_and_map(0));
    assert_eq!(226, invert_and_map(1));
    assert_eq!(224, invert_and_map(2));
    assert_eq!(208, invert_and_map(10));
    assert_eq!(188, invert_and_map(20));
    assert_eq!(147, invert_and_map(40));
    assert_eq!(127, invert_and_map(50));
    assert_eq!(107, invert_and_map(60));
    assert_eq!(86, invert_and_map(70));
    assert_eq!(66, invert_and_map(80));
    assert_eq!(46, invert_and_map(90));
    assert_eq!(26, invert_and_map(100));
}