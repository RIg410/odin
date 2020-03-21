mod serial;
mod web;

pub use self::serial::{SerialDimmer, SerialSwitch};
pub use self::web::{LedMode, LedState, WebBeam, WebSwitch};
use anyhow::Result;
use serde_json::Value;
use std::fmt::Debug;

pub trait Flush {
    fn flush(&self) -> Result<()>;
}

pub trait Switch {
    fn is_on(&self) -> bool;
    fn switch(&self, is_on: bool) -> Result<()>;
}

pub trait Control: Send + Sync + Debug {
    fn id(&self) -> &str;
    fn dev_type(&self) -> DeviceType;
    fn load(&self) -> Value;
    fn update(&self, state: Value) -> Result<()>;
}

pub enum DeviceType {
    SerialSwitch,
    SerialDimmer,
    WebBeam,
    WebSwitch,
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
