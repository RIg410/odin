use controller::Device;
use std::sync::Arc;

#[derive(Debug)]
pub struct Switch {
    pub id: String,
    pub devices: Vec<Arc<Device>>,
}

impl Switch {
    pub fn new(id: &str, devices: Vec<Arc<Device>>) -> Switch {
        Switch {id:id.to_owned(), devices }
    }
}