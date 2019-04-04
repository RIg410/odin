use transport::Transport;
use sensors::ActionType;
use std::sync::atomic::{AtomicBool, Ordering};
use transport::serial::Cmd;
use std::sync::Mutex;

pub trait Flush {
    fn flush(&self);
}

pub trait Switch {
    fn is_on(&self) -> bool;
    fn switch(&self, action_type: ActionType) -> bool;
}

#[derive(Debug)]
pub struct SerialSwitch {
    id: String,
    p_id: u8,
    transport: Transport,

    is_on: AtomicBool,
}

impl SerialSwitch {
    pub fn new(transport: &Transport, id: &str, p_id: u8) -> SerialSwitch {
        SerialSwitch {
            id: id.to_owned(),
            transport: transport.clone(),
            p_id,
            is_on: AtomicBool::new(false),
        }
    }
}

impl Flush for SerialSwitch {
    fn flush(&self) {
        let arg = if self.is_on.load(Ordering::SeqCst) {
            0x01
        } else {
            0x02
        };
        self.transport.serial_write(Cmd::new(0x02, self.p_id, arg));
    }
}

#[derive(Debug)]
pub struct SerialDimmer {
    id: String,
    p_id: u8,
    transport: Transport,
    min_value: u8,
    max_value: u8,
    state: Mutex<DimmerState>,
}

impl SerialDimmer {
    pub fn new(transport: &Transport, id: &str, p_id: u8, min_value: u8, max_value: u8) -> SerialDimmer {
        SerialDimmer {
            id: id.to_owned(),
            transport: transport.clone(),
            p_id,
            min_value,
            max_value,
            state: Mutex::new(DimmerState { is_on: false, brightness: 100 }),
        }
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

        self.transport.serial_write(Cmd::new(0x01, self.p_id, arg));
    }
}

#[derive(Debug)]
pub struct WebSwitch {
    id: String,
    transport: Transport,
    is_on: AtomicBool,
}

impl WebSwitch {
    pub fn new(transport: &Transport, id: &str) -> WebSwitch {
        WebSwitch {
            transport: transport.clone(),
            id: id.to_owned(),
            is_on: AtomicBool::new(false),
        }
    }
}

#[derive(Debug)]
pub struct WebBeam {
    id: String,
    transport: Transport,

    is_on: AtomicBool,
}

impl WebBeam {
    pub fn new(transport: &Transport, id: &str) -> WebBeam {
        WebBeam {
            transport: transport.clone(),
            id: id.to_owned(),
            is_on: AtomicBool::new(false),
        }
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