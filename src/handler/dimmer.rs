use controller::Dimmer as DimmableDevice;
use std::collections::HashMap;
use controller::SerialDimmer;

pub struct Dimmer {
    lamps: HashMap<String, Box<dyn DimmableDevice>>
}

impl Dimmer {
    pub fn new() -> Dimmer {
        Dimmer { lamps: HashMap::new() }
    }

    pub fn add_serial_dimmer(&mut self, dimmer: SerialDimmer) {
        self.lamps.insert(dimmer.id.as_ref().to_owned(), Box::new(dimmer));
    }

    pub fn dimm(&self, name: &str, val: u8) {
        if let Some(lamp) = self.lamps.get(name) {
            lamp.set_dimm(val);
            lamp.flush();
        }
    }
}