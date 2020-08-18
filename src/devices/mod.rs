mod serial;
mod web;

pub use self::serial::{SerialDimmer, SerialSwitch};
pub use self::web::{LedMode, LedState, WebBeam, WebSwitch};
use crate::runtime::time_ms;
use anyhow::Result;
use serde_json::Value;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};

pub trait Flush {
    fn flush(&self) -> Result<()>;
}

pub trait Switch {
    fn is_on(&self) -> bool;
    fn switch(&self, is_on: bool) -> Result<()>;
    fn toggle(&self) -> Result<()> {
        self.switch(!self.is_on())
    }
}

pub trait State {
    fn load(&self) -> Value;
    fn update(&self, state: Value) -> Result<()>;
}

pub trait LastUpdate {
    fn last_update(&self) -> u128;
}

pub trait Device: Send + Sync + Debug + Flush + State + Switch + LastUpdate {
    fn id(&self) -> &str;
    fn dev_type(&self) -> DeviceType;
}

pub enum DeviceType {
    SerialSwitch,
    SerialDimmer,
    WebBeam,
    WebSwitch,
}

#[derive(Clone, Debug)]
pub struct Stopwatch {
    time: Arc<RwLock<u128>>,
}

impl Stopwatch {
    pub fn new() -> Stopwatch {
        Stopwatch {
            time: Arc::new(RwLock::new(time_ms())),
        }
    }

    pub fn reset(&self) {
        *self.time.write().unwrap() = time_ms();
    }
}

impl LastUpdate for Stopwatch {
    fn last_update(&self) -> u128 {
        *self.time.read().unwrap()
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
