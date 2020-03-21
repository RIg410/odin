use anyhow::{Error, Result};
use std::{
    io::prelude::*,
    process::Command,
    sync::{Arc, Mutex},
    time::Duration,
};
use uart::{self, prelude::*, unix::TTYPort};

const SETTINGS: uart::PortSettings = uart::PortSettings {
    baud_rate: uart::Baud9600,
    char_size: uart::Bits8,
    parity: uart::ParityNone,
    stop_bits: uart::Stop1,
    flow_control: uart::FlowNone,
};

pub struct SerialChannel {
    port: Arc<Mutex<Option<TTYPort>>>,
}

impl SerialChannel {
    pub fn new() -> SerialChannel {
        SerialChannel {
            port: Arc::new(Mutex::new(None)),
        }
    }

    pub fn make_port(&self) -> Option<TTYPort> {
        get_port_name().and_then(|p| match uart::open(&p) {
            Ok(mut port) => {
                if let Err(err) = port.configure(&SETTINGS) {
                    info!("Failed to config port [{}] {:?}", p, err);
                }

                if let Err(err) = port.set_timeout(Duration::from_secs(1)) {
                    info!("Failed to set timeout [{}] {:?}", p, err);
                }
                Some(port)
            }
            Err(err) => {
                info!("Failed to open port.:{:?}", err);
                None
            }
        })
    }

    pub fn send(&self, cmd: Cmd) -> Result<()> {
        if let Ok(mut lock) = self.port.lock() {
            if lock.is_none() {
                *lock = self.make_port();
            }

            if lock.is_some() {
                let Cmd { _type, id, args } = cmd;
                if let Err(res) = lock.as_mut().unwrap().write(&[_type, id, args]) {
                    *lock = None;
                    Err(Error::msg(format!("failed to send {:?}", res)))
                } else if let Err(res) = lock.as_mut().unwrap().flush() {
                    *lock = None;
                    Err(Error::msg(format!("failed to flush {:?}", res)))
                } else {
                    Ok(())
                }
            } else {
                Err(Error::msg("Failed to send cmd. Failed to open port."))
            }
        } else {
            Err(Error::msg("Failed to get SerialChannel lock"))
        }
    }
}

pub fn get_port_name() -> Option<String> {
    let out = Command::new("sh")
        .arg("-c")
        .arg("ls /dev/serial/by-id/usb-Arduino_*")
        .output()
        .expect("failed to execute process");
    Some(
        String::from_utf8_lossy(&out.stdout)
            .to_owned()
            .replace("\n", ""),
    )
}

impl Clone for SerialChannel {
    fn clone(&self) -> Self {
        SerialChannel {
            port: self.port.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Cmd {
    _type: u8,
    id: u8,
    args: u8,
}

impl Cmd {
    pub fn new(_type: u8, id: u8, args: u8) -> Cmd {
        Cmd { _type, id, args }
    }
}
