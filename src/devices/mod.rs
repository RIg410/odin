mod beam;

pub use self::beam::WebBeam;
use io::{IO, IOBuilder, Output};
use std::sync::atomic::{AtomicBool, Ordering};
use io::Cmd;
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::fmt::Debug;

pub trait Flush {
    fn flush(&self);
}

pub trait Switch {
    fn is_on(&self) -> bool;
    fn switch(&self, is_on: bool);
}

pub trait Update: Send + Sync + Debug {
    fn id(&self) -> &str;
    fn update(&self, state: HashMap<String, String>) -> Result<(), String>;
}

#[derive(Debug, Clone)]
pub struct SerialSwitch {
    id: Arc<String>,
    p_id: u8,
    io: IO,
    is_on: Arc<AtomicBool>,
}

impl SerialSwitch {
    pub fn new(io: &mut IOBuilder, id: &str, p_id: u8) -> SerialSwitch {
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

impl Update for SerialSwitch {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn update(&self, state: HashMap<String, String>) -> Result<(), String> {
        unimplemented!()
    }
}

impl Flush for SerialSwitch {
    fn flush(&self) {
        let arg = if self.is_on.load(Ordering::SeqCst) {
            0x01
        } else {
            0x02
        };
        self.io.serial_write(Cmd::new(0x02, self.p_id, arg));
    }
}

#[derive(Debug, Clone)]
pub struct SerialDimmer {
    id: Arc<String>,
    p_id: u8,
    io: IO,
    min_value: u8,
    max_value: u8,
    state: Arc<Mutex<DimmerState>>,
}

impl SerialDimmer {
    pub fn new(io: &mut IOBuilder, id: &str, p_id: u8, min_value: u8, max_value: u8) -> SerialDimmer {
        let dev = SerialDimmer {
            id: Arc::new(id.to_owned()),
            io: io.shared(),
            p_id,
            min_value,
            max_value,
            state: Arc::new(Mutex::new(DimmerState { is_on: false, brightness: 100 })),
        };
        io.reg_device(Box::new(dev.clone()));

        dev
    }
}

impl Update for SerialDimmer {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn update(&self, state: HashMap<String, String>) -> Result<(), String> {
        unimplemented!()
    }
}

#[derive(Debug)]
struct DimmerState {
    is_on: bool,
    brightness: u8,
}

impl Flush for SerialDimmer {
    fn flush(&self) {
        let state = self.state.lock().unwrap();

        let arg = if state.is_on {
            invert_and_map(
                map(state.brightness as u32,
                    0,
                    100,
                    self.min_value as u32,
                    self.max_value as u32)
                    as u8)
        } else {
            255
        };

        self.io.serial_write(Cmd::new(0x01, self.p_id, arg));
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

impl Update for WebSwitch {
    fn id(&self) -> &str {
        self.id.as_str()
    }

    fn update(&self, state: HashMap<String, String>) -> Result<(), String> {
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