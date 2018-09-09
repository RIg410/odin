use serial;
use std::io::prelude::*;
use serial::prelude::*;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serial::unix::TTYPort;
use std::process::Command;

const SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud9600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

pub struct SerialChannel {
    port: Arc<Mutex<Option<TTYPort>>>
}

impl SerialChannel {
    pub fn new() -> SerialChannel {
        SerialChannel { port: Arc::new(Mutex::new(None)) }
    }

    pub fn make_port(&self) -> Option<TTYPort> {
        get_port_name()
            .and_then(|p| {
                match serial::open(&p) {
                    Ok(mut port) => {
                        if let Err(err) = port.configure(&SETTINGS) {
                            println!("Failed to config port [{}] {:?}", p, err);
                        }

                        if let Err(err) = port.set_timeout(Duration::from_secs(1)) {
                            println!("Failed to set timeout [{}] {:?}", p, err);
                        }
                        Some(port)
                    }
                    Err(err) => {
                        println!("Failed to open port.:{:?}", err);
                        None
                    }
                }
            })
    }

    pub fn send(&self, cmd: Cmd) -> bool {
        if let Ok(mut lock) = self.port.lock() {
            if lock.is_none() {
                *lock = self.make_port();
            }

            if lock.is_some() {
                let Cmd { _type, id, args } = cmd;
                if let Err(res) = lock.as_mut().unwrap().write(&[_type, id, args]) {
                    println!("failed to send {:?}", res);
                    *lock = None;
                } else {
                    if let Err(res) = lock.as_mut().unwrap().flush() {
                        println!("failed to flush {:?}", res);
                        *lock = None;
                    } else {
                        return true;
                    }
                }
            } else {
                println!("Failed to send cmd. Failed to open port.");
            }
        } else {
            println!("Failed to get SerialChannel lock");
        }
        false
    }
}

pub fn get_port_name() -> Option<String> {
    let out = Command::new("sh")
        .arg("-c")
        .arg("ls /dev/serial/by-id/usb-Arduino_*")
        .output()
        .expect("failed to execute process");
    Some(String::from_utf8_lossy(&out.stdout).to_owned().replace("\n", ""))
}

impl Clone for SerialChannel {
    fn clone(&self) -> Self {
        SerialChannel { port: self.port.clone() }
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