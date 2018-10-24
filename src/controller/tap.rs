use std::sync::atomic::{AtomicBool, Ordering};
use controller::{Device, Message, ControllerError};
use gpio::Raspberry;
use transport::MqttChannel;
use rppal::gpio::{Mode, Level};
use std::thread;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug)]
pub struct Tap {
    id: String,
    // true on, false off
    state: AtomicBool,
    gpio: Raspberry,
    pin_set: (u8, u8),
}

impl Tap {
    pub fn new(id: &str, gpio: Raspberry, pin_set: (u8, u8)) -> Tap {
        Tap { id: id.to_owned(), state: AtomicBool::new(false), gpio, pin_set }
    }
}

impl Device for Tap {
    fn is_on(&self) -> Result<bool, ControllerError> {
        Ok(self.state.load(Ordering::SeqCst))
    }

    fn is_off(&self) -> Result<bool, ControllerError> {
        Ok(!self.state.load(Ordering::SeqCst))
    }

    fn on(&self) -> Result<(), ControllerError> {
        self.state.store(true, Ordering::SeqCst);
        Ok(())
    }

    fn off(&self) -> Result<(), ControllerError> {
        self.state.store(false, Ordering::SeqCst);
        Ok(())
    }

    fn toggle(&self) -> Result<bool, ControllerError> {
        loop {
            let old = self.state.load(Ordering::SeqCst);
            if self.state.compare_and_swap(old, !old, Ordering::SeqCst) == old {
                return Ok(!old);
            }
        }
    }

    fn flush(&self) -> Result<(), ControllerError> {
        if self.state.load(Ordering::SeqCst) {
            //open
            self.gpio.sync_access(|ref mut gpio| {
                gpio.set_mode(self.pin_set.0, Mode::Output);
                gpio.set_mode(self.pin_set.1, Mode::Output);

                gpio.write(self.pin_set.0, Level::High);
                gpio.write(self.pin_set.1, Level::Low);
                thread::sleep_ms(10 * 1000);
                gpio.write(self.pin_set.0, Level::Low);
            });
        } else {
            //close
            self.gpio.sync_access(|ref mut gpio| {
                gpio.set_mode(self.pin_set.0, Mode::Output);
                gpio.set_mode(self.pin_set.1, Mode::Output);

                gpio.write(self.pin_set.1, Level::High);
                gpio.write(self.pin_set.0, Level::Low);
                thread::sleep_ms(10 * 1000);
                gpio.write(self.pin_set.1, Level::Low);
            });
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct Taps {
    taps: Arc<HashMap<String, Tap>>
}

impl Taps {
    pub fn new(taps: Vec<Tap>) -> Taps {
        Taps {
            taps: Arc::new(taps.into_iter().map(|t| (t.id.to_owned(), t)).collect())
        }
    }

    pub fn switch_on(&self, id: &str) {
        if let Some(tap) = self.taps.get(id) {
            tap.on();
            tap.flush();
        }
    }

    pub fn switch_off(&self, id: &str) {
        if let Some(tap) = self.taps.get(id) {
            tap.off();
            tap.flush();
        }
    }
}

impl Clone for Taps {
    fn clone(&self) -> Self {
        Taps {
            taps: self.taps.clone()
        }
    }
}