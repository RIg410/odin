use std::sync::atomic::{AtomicBool, Ordering};
use controller::{Device, Message, ControllerError};
use transport::MqttChannel;

#[derive(Debug)]
pub struct Tap {
    id: String,
    // true on, false off
    state: AtomicBool,
    channel: MqttChannel,
}

impl Tap {
    pub fn new(id: &str, channel: MqttChannel) -> Tap {
        Tap { id: id.to_owned(), state: AtomicBool::new(false), channel }
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
        let state = if self.state.load(Ordering::SeqCst) {
            0x01u8
        } else {
            0x00u8
        };

        self.channel.publish(Message::new(&format!("/odin/tap/{}/", self.id), vec!(state)))?;
        Ok(())
    }
}