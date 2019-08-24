mod web;
mod serial;

pub use self::web::{WebBeam, WebSwitch, LedState, LedMode};
pub use self::serial::{SerialSwitch, SerialDimmer};
use std::fmt::Debug;
use serde_json::Value;

pub trait Flush {
    fn flush(&self);
}

pub trait Switch {
    fn is_on(&self) -> bool;
    fn switch(&self, is_on: bool);
}

pub trait Control: Send + Sync + Debug {
    fn id(&self) -> &str;
    fn dev_type(&self) -> DeviceType;
    fn load(&self) -> Value;
    fn update(&self, state: Value) -> Result<(), String>;
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