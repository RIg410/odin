use rppal::gpio::{Gpio, Mode, Level};
use std::sync::Mutex;
use std::sync::Arc;
use std::fmt::{Debug, Result};
use std::fmt::Formatter;

pub struct Raspberry {
gpio: Arc<Mutex<Gpio>>,
}

impl Raspberry {
    pub fn new() -> Raspberry {
        Raspberry {
            gpio: Arc::new(Mutex::new(Gpio::new().unwrap()))
        }
    }

    pub fn sync_access<F>(&self, f: F) where F: Fn(&mut Gpio) {
       let mut gpio = self.gpio.lock().unwrap();
        f(&mut gpio);
    }
}

impl Clone for Raspberry {
    fn clone(&self) -> Self {
        Raspberry {
            gpio: self.gpio.clone()
        }
    }
}

impl Debug for Raspberry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "Raspberry {{}}")
    }
}