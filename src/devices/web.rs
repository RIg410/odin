use crate::devices::{Control, DeviceType, Flush, Switch};
use crate::io::{IOBuilder, Output, IO};
use anyhow::{Result, Error};
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub type Color = (u8, u8, u8);
pub type SpeedAndBrightness = (u8, u8);

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Noise {
    hue_start: u8,
    hue_gap: u8,
    noise_step: u8,
    min_bright: u8,
    max_bright: u8,
    min_sat: u8,
    max_sat: u8,
    delay: u8,
}

impl Noise {
    //    pub fn new(hue_start: u8, hue_gap: u8, noise_step: u8) -> Noise {
    //        let mut noise = Self::default();
    //        noise.hue_gap = hue_gap;
    //        noise.noise_step = noise_step;
    //        noise.hue_start = hue_start;
    //        noise
    //    }
    //    pub fn preset_1() -> Noise {
    //        Noise {
    //            hue_start: 0,
    //            hue_gap: 50,
    //            noise_step: 50,
    //            min_bright: 245,
    //            max_bright: 255,
    //            min_sat: 245,
    //            max_sat: 255,
    //            delay: 40,
    //        }
    //    }
    //
    //    pub fn preset_2() -> Noise {
    //        Noise {
    //            hue_start: 180,
    //            hue_gap: 255,
    //            noise_step: 50,
    //            min_bright: 100,
    //            max_bright: 255,
    //            min_sat: 250,
    //            max_sat: 255,
    //            delay: 40,
    //        }
    //    }
}

impl Default for Noise {
    fn default() -> Self {
        Noise {
            hue_start: 0,
            hue_gap: 21,
            noise_step: 15,
            min_bright: 150,
            max_bright: 255,
            min_sat: 245,
            max_sat: 255,
            delay: 40,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum LedMode {
    Color(Color),
    Rainbow(SpeedAndBrightness),
    Borealis(SpeedAndBrightness),
    Noise(Noise),
}

impl LedMode {
    fn arg(&self) -> String {
        match self {
            LedMode::Color((r, g, b)) => format!("color:{}:{}:{}", r, g, b),
            LedMode::Rainbow((speed, power)) => format!("rainbow:{}:{}", speed, power),
            LedMode::Borealis((speed, power)) => format!("borealis:{}:{}", speed, power),
            LedMode::Noise(n) => format!(
                "noise:{}:{}:{}:{}:{}:{}:{}:{}",
                n.hue_start,
                n.hue_gap,
                n.noise_step,
                n.min_bright,
                n.max_bright,
                n.min_sat,
                n.max_sat,
                n.delay
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LedState {
    pub is_on: bool,
    pub mode: LedMode,
}

impl Default for LedState {
    fn default() -> Self {
        LedState {
            is_on: true,
            mode: LedMode::Rainbow((100, 100)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct BeamState {
    is_on: bool,
    led_state: LedState,
    is_spot_on: bool,
}

impl BeamState {
    pub fn set_state(&mut self, spot: Option<bool>, led: Option<LedState>) {
        if let Some(spot) = spot {
            self.is_spot_on = spot;
        }

        if let Some(led) = led {
            self.led_state = led;
        }
    }

    fn args(&self) -> String {
        let spot_state = if self.is_on && self.is_spot_on {
            "ON"
        } else {
            "OFF"
        };
        let led_stat = if self.is_on && self.led_state.is_on {
            "ON"
        } else {
            "OFF"
        };
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
            channel_1: Arc::new(RwLock::new(BeamState {
                is_on: false,
                led_state: LedState::default(),
                is_spot_on: true,
            })),
            id: Arc::new(id.to_owned()),
            channel_2: Arc::new(RwLock::new(BeamState {
                is_on: false,
                led_state: LedState::default(),
                is_spot_on: true,
            })),
        };
        io.reg_device(Box::new(dev.clone()));

        dev
    }

    pub fn channel_1(&self, spot: Option<bool>, led: Option<LedState>) {
        self.channel_1.write().unwrap().set_state(spot, led);
    }

    pub fn channel_2(&self, spot: Option<bool>, led: Option<LedState>) {
        self.channel_2.write().unwrap().set_state(spot, led);
    }
}

impl Switch for WebBeam {
    fn is_on(&self) -> bool {
        self.channel_1.read().unwrap().is_on || self.channel_2.read().unwrap().is_on
    }

    fn switch(&self, is_on: bool) -> Result<()> {
        let args = {
            let mut channel_1 = self.channel_1.write().unwrap();
            let mut channel_2 = self.channel_2.write().unwrap();
            channel_1.is_on = is_on;
            channel_2.is_on = is_on;
            vec![channel_1.args(), channel_2.args()]
        };
        self.io.send(&self.id, args)
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

    fn update(&self, state: Value) -> Result<()> {
        let state: WebBeamState = serde_json::from_value(state)?;
        {
            *self.channel_1.write().unwrap() = state.channel_1;
            *self.channel_2.write().unwrap() = state.channel_2;
        }
        if let Some(is_on) = state.is_on {
            self.switch(is_on)?;
        }
        Ok(())
    }
}

impl Flush for WebBeam {
    fn flush(&self) -> Result<(), Error> {
        let args = {
            let channel_1 = self.channel_1.read().unwrap();
            let channel_2 = self.channel_2.read().unwrap();
            vec![channel_1.args(), channel_2.args()]
        };
        self.io.send(&self.id, args)
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

    fn switch(&self, is_on: bool) -> Result<()> {
        self.is_on.store(is_on, Ordering::SeqCst);
        self.flush()
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

    fn update(&self, state: Value) -> Result<()> {
        if let Some(is_on) = &state["is_on"].as_bool() {
            self.switch(is_on.to_owned())?;
        }
        Ok(())
    }
}

impl Flush for WebSwitch {
    fn flush(&self) -> Result<()> {
        let is_on = self.is_on.load(Ordering::SeqCst);
        let arg = format!("{}:{}", if is_on { "ON" } else { "OFF" }, 100);
        self.io.send(&self.id, vec![arg])
    }
}
