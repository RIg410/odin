use std::sync::{Arc, RwLock};
use io::{IO, IOBuilder, Output};
use std::collections::HashMap;
use devices::{Control, Switch, Flush, DeviceType};
use std::sync::atomic::{AtomicBool, Ordering};
use serde_json::Value;

pub type Color = (u8, u8, u8);
pub type SpeedAndBrightness = (u8, u8);

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LedState {
    is_on: bool,
    mode: LedMode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize)]
struct WebBeamState {
    is_on: Option<bool>,
    channel_1: BeamState,
    channel_2: BeamState,
}

///
/// state {is_on, channel_1:"{is_on, is_spot_on, led:"{}"}", channel_2:"{}"}
///
impl Control for WebBeam {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn dev_type(&self) -> DeviceType {
        DeviceType::WebBeam
    }

    fn load(&self) -> Value {
        let state = WebBeamState {
            is_on: Some(self.is_on()),
            channel_1: self.channel_1.read().unwrap().clone(),
            channel_2: self.channel_2.read().unwrap().clone(),
        };
        serde_json::to_value(&state).unwrap()
    }

    fn update(&self, state: Value) -> Result<(), String> {
        let state: WebBeamState = serde_json::from_value(state)
            .map_err(|err| format!("Failed to parse json: {}", err))?;
        {
            *self.channel_1.write().unwrap() = state.channel_1;
            *self.channel_2.write().unwrap() = state.channel_2;
        }
        if let Some(is_on) = state.is_on {
            self.switch(is_on);
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct WebSwitch {
    id: Arc<String>,
    io: IO,
    is_on: Arc<AtomicBool>,
}

impl WebSwitch {
    pub fn new(io: &mut IOBuilder, id: &str) -> WebSwitch {
        let dev = WebSwitch {
            io: io.shared(),
            id: Arc::new(id.to_owned()),
            is_on: Arc::new(AtomicBool::new(false)),
        };
        io.reg_device(Box::new(dev.clone()));

        dev
    }
}

impl Switch for WebSwitch {
    fn is_on(&self) -> bool {
        self.is_on.load(Ordering::SeqCst)
    }

    fn switch(&self, is_on: bool) {
        self.is_on.store(is_on, Ordering::SeqCst);
        self.flush();
    }
}

impl Control for WebSwitch {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn dev_type(&self) -> DeviceType {
        DeviceType::WebSwitch
    }

    fn load(&self) -> Value {
        json!({
            "is_on": self.is_on.load(Ordering::SeqCst)
        })
    }

    fn update(&self, state: Value) -> Result<(), String> {
        if let Some(is_on) = &state["is_on"].as_bool() {
            self.switch(is_on.to_owned());
        }
        Ok(())
    }
}

impl Flush for WebSwitch {
    fn flush(&self) {
        let is_on = self.is_on.load(Ordering::SeqCst);
        let arg = format!("{}:{}",
                          if is_on { "ON" } else { "OFF" },
                          100);
        self.io.send(&self.id, vec![arg]);
    }
}