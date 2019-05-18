use std::sync::{Arc, RwLock};
use io::{IO, IOBuilder, Output};
use std::collections::HashMap;
use devices::{Update, Switch};

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

#[derive(Debug)]
struct BeamState {
    is_on: bool,
    led_state: LedState,
    is_spot_on: bool,
}

impl BeamState {
    fn args(&self) -> String {
        let spot_state = if self.is_spot_on { "ON" } else { "OFF" };
        let led_stat = if self.led_state.is_on { "ON" } else { "OFF" };
        format!("{}:{}:{}", spot_state, led_stat, self.led_state.mode.arg())
    }
}

#[derive(Debug, Clone)]
pub struct WebBeam {
    id: Arc<String>,
    io: IO,
    channel_1: Arc<RwLock<BeamState>>,
    channel_2: Arc<RwLock<BeamState>>,
}

impl WebBeam {
    pub fn new(io: &mut IOBuilder, id: &str) -> WebBeam {
        let dev = WebBeam {
            io: io.shared(),
            channel_1: Arc::new(RwLock::new(BeamState { is_on: false, led_state: LedState { is_on: false, mode: LedMode::Rainbow((100, 100)) }, is_spot_on: false })),
            id: Arc::new(id.to_owned()),
            channel_2: Arc::new(RwLock::new(BeamState { is_on: false, led_state: LedState { is_on: false, mode: LedMode::Rainbow((100, 100)) }, is_spot_on: false })),
        };
        io.reg_device(Box::new(dev.clone()));

        dev
    }
}

impl Switch for WebBeam {
    fn is_on(&self) -> bool {
        self.channel_1.read().unwrap().is_on || self.channel_2.read().unwrap().is_on
    }

    fn switch(&self, is_on: bool) {
        let mut channel_1 = self.channel_1.write().unwrap();
        let mut channel_2 = self.channel_1.write().unwrap();
        channel_1.is_on = is_on;
        channel_2.is_on = is_on;

        self.io.send(&self.id, vec![channel_1.args(), channel_2.args()]);
    }
}

impl Update for WebBeam {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn update(&self, state: HashMap<String, String>) -> Result<(), String> {
        unimplemented!()
    }
}