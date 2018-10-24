use controller::Device;
use std::sync::Arc;

pub trait Switch {
    fn devices(&self) -> &Vec<Box<Device>>;
    fn id(&self) -> &str;
    fn switch_on(&self);
    fn switch_off(&self);
}

#[derive(Debug)]
pub struct CommonSwitch {
    inner: Arc<Inner>
}

#[derive(Debug)]
struct Inner {
    pub id: String,
    pub devices: Vec<Box<Device>>,
}

impl CommonSwitch {
    pub fn new(id: &str, devices: Vec<Box<Device>>) -> CommonSwitch {
        CommonSwitch { inner: Arc::new(Inner { id: id.to_owned(), devices }) }
    }
}

impl Switch for CommonSwitch {
    fn devices(&self) -> &Vec<Box<Device>> {
        &self.inner.devices
    }

    fn id(&self) -> &str {
        &self.inner.id
    }

    fn switch_on(&self) {
        for dev in &self.inner.devices {
            dev.on();
            if let Err(err) = dev.flush() {
                println!("Failed to send: {:?}", err);
            }
        }
    }

    fn switch_off(&self) {
        for dev in &self.inner.devices {
            dev.off();
            dev.flush();
        }
    }
}

impl Clone for CommonSwitch {
    fn clone(&self) -> Self {
        CommonSwitch { inner: self.inner.clone() }
    }
}

#[derive(Debug)]
pub struct ExitSwitch {
    inner: Arc<Inner>
}

impl ExitSwitch {
    pub fn new(id: &str, devices: Vec<Box<Device>>) -> ExitSwitch {
        ExitSwitch { inner: Arc::new(Inner { id: id.to_owned(), devices }) }
    }
}

impl Switch for ExitSwitch {
    fn devices(&self) -> &Vec<Box<Device>> {
        &self.inner.devices
    }

    fn id(&self) -> &str {
        &self.inner.id
    }

    fn switch_on(&self) {
        self.switch_off();
    }

    fn switch_off(&self) {
        for dev in &self.inner.devices {
            dev.off();
            dev.flush();
        }
    }
}

impl Clone for ExitSwitch {
    fn clone(&self) -> Self {
        ExitSwitch { inner: self.inner.clone() }
    }
}