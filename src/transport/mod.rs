use transport::serial::SerialChannel;
use std::fmt::{Debug, Formatter, Error};
use transport::web::WebChannel;

//TODO remove public semantic
pub mod serial;
pub mod web;

#[derive(Clone)]
pub struct Transport {
    serial: SerialChannel,
    web: WebChannel,
}

impl Transport {
    pub fn create() -> Transport {
        Transport {
            serial: SerialChannel::new(),
            web: WebChannel::new(),
        }
    }
}

impl Debug for Transport {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Transport {{}}")
    }
}