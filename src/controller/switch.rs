use controller::Device;
use std::sync::Arc;

#[derive(Debug)]
pub struct Switch {
    inner: Arc<Inner>
}

#[derive(Debug)]
struct Inner {
    pub id: String,
    pub devices: Vec<Box<Device>>,
}

impl Switch {
    pub fn new(id: &str, devices: Vec<Box<Device>>) -> Switch {
        Switch { inner: Arc::new(Inner { id: id.to_owned(), devices }) }
    }

    pub fn devices(&self) -> &Vec<Box<Device>> {
        &self.inner.devices
    }

    pub fn id(&self) -> &str {
        &self.inner.id
    }

    pub fn switch_on(&self) {
        for dev in &self.inner.devices {
            dev.on();
            if let Err(err) = dev.flush() {
                println!("Failed to send: {:?}", err);
            }
        }
    }

    pub fn switch_off(&self) {
        for dev in &self.inner.devices {
            dev.off();
            dev.flush();
        }
    }
}

impl Clone for Switch {
    fn clone(&self) -> Self {
        Switch { inner: self.inner.clone() }
    }
}