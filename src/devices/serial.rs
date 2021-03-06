use crate::devices::{invert_and_map, map, Control, DeviceType, Flush, Switch};
use crate::io::{Cmd, IOMut, Output, IO};
use anyhow::Result;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SerialSwitch {
    id: Arc<String>,
    p_id: u8,
    io: IO,
    is_on: Arc<AtomicBool>,
}

impl SerialSwitch {
    pub fn new(io: &mut IOMut, id: &str, p_id: u8) -> SerialSwitch {
        let dev = SerialSwitch {
            id: Arc::new(id.to_owned()),
            io: io.shared(),
            p_id,
            is_on: Arc::new(AtomicBool::new(false)),
        };
        io.reg_device(Box::new(dev.clone()));
        dev
    }
}

impl Switch for SerialSwitch {
    fn is_on(&self) -> bool {
        self.is_on.load(Ordering::SeqCst)
    }

    fn switch(&self, is_on: bool) -> Result<()> {
        self.is_on.store(is_on, Ordering::SeqCst);
        self.flush()
    }
}

///
/// State {is_on}
///
impl Control for SerialSwitch {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn dev_type(&self) -> DeviceType {
        DeviceType::SerialSwitch
    }

    fn load(&self) -> Value {
        json!({
            "is_on": self.is_on.load(Ordering::SeqCst)
        })
    }

    fn update(&self, state: Value) -> Result<()> {
        if let Some(is_on) = &state["is_on"].as_bool() {
            self.switch(is_on.to_owned())
        } else {
            Ok(())
        }
    }
}

impl Flush for SerialSwitch {
    fn flush(&self) -> Result<()> {
        let arg = if self.is_on.load(Ordering::SeqCst) {
            0x01
        } else {
            0x02
        };
        self.io.serial_write(Cmd::new(0x02, self.p_id, arg))
    }
}

#[derive(Debug, Clone)]
pub struct SerialDimmer {
    id: Arc<String>,
    p_id: u8,
    io: IO,
    min_value: u8,
    max_value: u8,
    state: Arc<RwLock<DimmerState>>,
}

impl SerialDimmer {
    pub fn new(io: &mut IOMut, id: &str, p_id: u8, min_value: u8, max_value: u8) -> SerialDimmer {
        let dev = SerialDimmer {
            id: Arc::new(id.to_owned()),
            io: io.shared(),
            p_id,
            min_value,
            max_value,
            state: Arc::new(RwLock::new(DimmerState {
                is_on: false,
                brightness: 100,
            })),
        };
        io.reg_device(Box::new(dev.clone()));

        dev
    }

    pub fn set_power(&self, power: u8) {
        self.state.write().unwrap().brightness = power;
    }
}

impl Switch for SerialDimmer {
    fn is_on(&self) -> bool {
        self.state.read().unwrap().is_on
    }

    fn switch(&self, is_on: bool) -> Result<()> {
        {
            self.state.write().unwrap().is_on = is_on;
        }

        self.flush()
    }
}

///
/// State {is_on, brightness}
///
impl Control for SerialDimmer {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn dev_type(&self) -> DeviceType {
        DeviceType::SerialDimmer
    }

    fn load(&self) -> Value {
        let state = self.state.read().unwrap();

        json!({
            "is_on": state.is_on,
            "brightness": state.brightness
        })
    }

    fn update(&self, val: Value) -> Result<()> {
        if let Some(brightness) = val["brightness"].as_u64() {
            let mut state = self.state.write().unwrap();
            state.brightness = brightness as u8;
        }

        if let Some(is_on) = &val["is_on"].as_bool() {
            self.switch(is_on.to_owned())
        } else {
            self.flush()
        }
    }
}

#[derive(Debug)]
struct DimmerState {
    is_on: bool,
    brightness: u8,
}

impl Flush for SerialDimmer {
    fn flush(&self) -> Result<()> {
        let state = self.state.read().unwrap();

        let arg = if state.is_on {
            invert_and_map(map(
                state.brightness as u32,
                0,
                100,
                self.min_value as u32,
                self.max_value as u32,
            ) as u8)
        } else {
            255
        };

        self.io.serial_write(Cmd::new(0x01, self.p_id, arg))
    }
}
