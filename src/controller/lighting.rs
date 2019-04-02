use super::Device;
use std::{
    sync::{Arc, RwLock},
    fmt,
};
use serial::{SerialChannel, Cmd};
use controller::ActionType;
use web::WebController;
use thread;

#[derive(Debug, Eq, PartialEq)]
struct SpotState {
    is_on: bool,
    brightness: u8,
    old_brightness: u8,
}

impl SpotState {
    fn new() -> SpotState {
        SpotState { is_on: false, brightness: 100, old_brightness: 0 }
    }
}

pub struct SerialDimmer {
    pub id: Arc<String>,
    p_id: u8,
    channel: SerialChannel,
    state: Arc<RwLock<SpotState>>,
    can_dim: bool,
    min: usize,
    max: usize,
}

impl Clone for SerialDimmer {
    fn clone(&self) -> Self {
        SerialDimmer {
            id: self.id.clone(),
            p_id: self.p_id.clone(),
            channel: self.channel.clone(),
            state: self.state.clone(),
            can_dim: self.can_dim.clone(),
            min: self.min,
            max: self.max,
        }
    }
}

impl SerialDimmer {
    pub fn switch(id: &str, p_id: u8, channel: &SerialChannel) -> SerialDimmer {
        SerialDimmer {
            id: Arc::new(id.to_owned()),
            p_id,
            channel: channel.clone(),
            state: Arc::new(RwLock::new(SpotState::new())),
            can_dim: false,
            min: 0,
            max: 100,
        }
    }

    pub fn dimmer(id: &str, p_id: u8, channel: &SerialChannel, min: usize, max: usize) -> SerialDimmer {
        SerialDimmer {
            id: Arc::new(id.to_owned()),
            p_id,
            channel: channel.clone(),
            state: Arc::new(RwLock::new(SpotState::new())),
            can_dim: true,
            min,
            max,
        }
    }

    fn flush(&self) {
        let mut state = self.state.write().unwrap();

        if self.can_dim {
            loop {
                let arg = if state.is_on {
                    invert_and_map(map(state.old_brightness as u32, 0, 100, self.min as u32, self.max as u32) as u8)
                } else {
                    255
                };

                self.channel.send(Cmd::new(0x01, self.p_id, arg));
                if state.old_brightness == state.brightness {
                    return;
                }
                thread::sleep_ms(50);

                if state.old_brightness < state.brightness {
                    if state.brightness - state.old_brightness <= 2 {
                        state.old_brightness = state.brightness;
                    } else {
                        state.old_brightness += 2;
                    }
                } else {
                    if state.old_brightness - state.brightness <= 2 {
                        state.old_brightness = state.brightness;
                    } else {
                        state.old_brightness -= 2;
                    }
                }
            }
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
        {
            let mut state = self.state.write().unwrap();
            state.is_on = action_type == &ActionType::On;
        }
        self.flush()
    }

    fn set_power(&self, dim: u8) {
        {
            let mut state = self.state.write().unwrap();
            state.old_brightness = state.brightness;
            state.brightness = dim;
        }
        self.flush()
    }

    fn set_state(&self, action_type: &ActionType, power: u8) {
        {
            let mut state = self.state.write().unwrap();
            state.old_brightness = state.brightness;
            state.brightness = power;
            state.is_on = action_type == &ActionType::On;
        }
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
    web: WebController,
}

impl WebDimmer {
    pub fn new(id: &str, web: &WebController) -> WebDimmer {
        WebDimmer {
            id: Arc::new(id.to_owned()),
            state: Arc::new(RwLock::new(SpotState::new())),
            web: web.clone(),
        }
    }

    pub fn flush(&self) {
        let state = self.state.read().unwrap();
        let switch = if state.is_on { "ON" } else { "OFF" };
        let arg = format!("{}:{}", switch, state.brightness);
        self.web.send(&self.id, &arg, &arg);
    }
}

impl Device for WebDimmer {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_on(&self) -> bool {
        self.state.read().unwrap().is_on
    }

    fn power(&self) -> u8 {
        let state = self.state.read().unwrap();
        state.brightness
    }

    fn switch(&self, action_type: &ActionType) {
        {
            let mut state = self.state.write().unwrap();
            state.is_on = action_type == &ActionType::On;
        }
        self.flush()
    }

    fn set_power(&self, _power: u8) {
        self.flush()
    }

    fn set_state(&self, action_type: &ActionType, power: u8) {
        {
            let mut state = self.state.write().unwrap();
            state.brightness = power;
            state.is_on = action_type == &ActionType::On;
        }
        self.flush()
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

impl LedMode {
    fn arg(&self) -> String {
        match self {
            LedMode::Color((r, g, b)) => format!("color:{}:{}:{}", r, g, b),
            LedMode::Rainbow((speed, power)) => format!("rainbow:{}:{}", speed, power),
            LedMode::Borealis((speed, power)) => format!("borealis:{}:{}", speed, power),
        }
    }
}

#[derive(Debug)]
pub struct LedState {
    is_on: bool,
    mode: LedMode,
}

#[derive(Debug, Clone)]
pub struct WebLed {
    pub id: Arc<String>,
    state: Arc<RwLock<LedState>>,
    web: WebController,
}

impl WebLed {
    pub fn new(id: &str, web: &WebController) -> WebLed {
        WebLed {
            id: Arc::new(id.to_owned()),
            state: Arc::new(RwLock::new(LedState { is_on: false, mode: LedMode::Rainbow((100, 100)) })),
            web: web.clone(),
        }
    }

    pub fn flush(&self) {
        let state = self.state.read().unwrap();
        let switch = if state.is_on { "ON" } else { "OFF" };
        let arg = format!("{}:{}", switch, state.mode.arg());
        self.web.send(&self.id, &arg, &arg);
    }
}

impl Device for WebLed {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_on(&self) -> bool {
        self.state.read().unwrap().is_on
    }

    fn power(&self) -> u8 {
        let state = self.state.read().unwrap();
        if state.is_on { 100 } else { 0 }
    }

    fn switch(&self, action_type: &ActionType) {
        {
            let mut state = self.state.write().unwrap();
            state.is_on = action_type == &ActionType::On;
        }
        self.flush();
    }

    fn set_power(&self, _power: u8) {
        self.flush();
    }

    fn set_state(&self, action_type: &ActionType, _power: u8) {
        {
            let mut state = self.state.write().unwrap();
            state.is_on = action_type == &ActionType::On;
        }
        self.flush()
    }
}

#[derive(Debug)]
struct BeamState {
    led_state: LedState,
    is_spot_on: bool,
}

#[derive(Debug, Clone)]
pub struct WebBeam {
    pub id: Arc<String>,
    state: Arc<RwLock<BeamState>>,
    web: WebController,
}

impl WebBeam {
    pub fn new(id: &str, web: &WebController) -> WebBeam {
        WebBeam {
            id: Arc::new(id.to_owned()),
            state: Arc::new(RwLock::new(BeamState {
                led_state: LedState { is_on: false, mode: LedMode::Rainbow((100, 100)) },
                is_spot_on: false,
            })),
            web: web.clone(),
        }
    }

    pub fn flush(&self) {
        let state = self.state.read().unwrap();
        let spot_state = if state.is_spot_on { "ON" } else { "OFF" };
        let led_stat = if state.led_state.is_on { "ON" } else { "OFF" };
        let arg = format!("{}:{}:{}", spot_state, led_stat, state.led_state.mode.arg());
        self.web.send(&self.id, &arg, &arg);
    }
}

impl Device for WebBeam {
    fn id(&self) -> &str {
        &self.id
    }

    fn is_on(&self) -> bool {
        self.state.read().unwrap().is_spot_on
    }

    fn power(&self) -> u8 {
        0
    }

    fn switch(&self, action_type: &ActionType) {
        {
            let mut state = self.state.write().unwrap();
            state.is_spot_on = action_type == &ActionType::On;
            state.led_state.is_on = action_type == &ActionType::On;
        }
        self.flush();
    }

    fn set_power(&self, power: u8) {}

    fn set_state(&self, action_type: &ActionType, power: u8) {
        self.switch(action_type);
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