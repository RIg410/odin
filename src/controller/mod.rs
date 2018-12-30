mod lighting;
mod switch;

pub use controller::{
    lighting::{SerialDimmer, WebDimmer, WebLed},
    switch::{Switch, SwitchHandler},
};
use std::{
    time::Duration,
    sync::PoisonError,
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, RwLock},
    str::FromStr,
    ops::{Add, AddAssign},
};
use std::fmt::Formatter;
use std::fmt::Error;
use std::ops::SubAssign;
use timer;
use std::sync::Mutex;

pub trait Device: Send + Sync + Debug {
    fn id(&self) -> &str;
    fn is_on(&self) -> bool;
    fn power(&self) -> u8;
    fn switch(&self, action_type: &ActionType);
    fn set_power(&self, power: u8);
    fn set_state(&self, action_type: &ActionType, power: u8);
}

#[derive(PartialEq)]
pub enum ActionType {
    On,
    Off,
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "ON" => {
                Ok(ActionType::On)
            }
            "OFF" => {
                Ok(ActionType::Off)
            }
            _ => Err(())
        }
    }
}

#[derive(Debug)]
pub enum ControllerError {
    GardError(String),
}

impl<T> From<PoisonError<T>> for ControllerError {
    fn from(err: PoisonError<T>) -> ControllerError {
        ControllerError::GardError(err.to_string())
    }
}

type Timer = Arc<Mutex<timer::Timer>>;

fn timer() -> Timer {
    Arc::new(Mutex::new(timer::Timer::new()))
}

#[derive(Clone, Debug)]
pub enum DeviceBox {
    SerialDimmer((SerialDimmer, Timer)),
    WebDimmer((WebDimmer, Timer)),
    WebLed((WebLed, Timer)),
}

impl DeviceBox {
    fn dev(&self) -> &Device {
        match self {
            DeviceBox::SerialDimmer((dev, timer)) => dev,
            DeviceBox::WebDimmer((dev, timer)) => dev,
            DeviceBox::WebLed((dev, timer)) => dev,
        }
    }

    fn timer(&self) -> &Timer {
        match self {
            DeviceBox::SerialDimmer((dev, timer)) => timer,
            DeviceBox::WebDimmer((dev, timer)) => timer,
            DeviceBox::WebLed((dev, timer)) => timer,
        }
    }

    pub fn reset_timer(&self) {
        let mut timer = self.timer().lock().unwrap();
        timer.reset();
    }

    pub fn delay<A>(&self, duration: Duration, action: A)
        where A: Fn(&DeviceBox) + 'static + Send + Sync {
        let mut timer = self.timer().lock().unwrap();
        let device = self.clone();
        timer.after(duration, move || {
            action(&device);
        });
    }
}

impl Device for DeviceBox {
    fn id(&self) -> &str {
        self.dev().id()
    }

    fn is_on(&self) -> bool {
        self.dev().is_on()
    }

    fn power(&self) -> u8 {
        self.dev().power()
    }

    fn switch(&self, action_type: &ActionType) {
        self.dev().switch(action_type);
        self.reset_timer();
    }

    fn set_power(&self, power: u8) {
        self.dev().set_power(power);
        self.reset_timer();
    }

    fn set_state(&self, action_type: &ActionType, power: u8) {
        self.dev().set_state(action_type, power);
        self.reset_timer();
    }
}

#[derive(Clone)]
pub struct DeviceHandler {
    devices: Arc<RwLock<HashMap<String, DeviceBox>>>
}

impl DeviceHandler {
    pub fn new() -> DeviceHandler {
        DeviceHandler {
            devices: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn dev(&self, id: &str) -> DeviceBox {
        let map = self.devices.read().unwrap();
        map.get(id).unwrap().clone()
    }

    pub fn push(&self, device: DeviceBox) -> DeviceBox {
        let mut map = self.devices.write().unwrap();
        map.insert(device.id().to_string(), device.clone());
        device
    }

    pub fn switch(&self, id: &str, action_type: ActionType) {
        let map = self.devices.read().unwrap();
        if let Some(device) = map.get(id) {
            device.switch(&action_type)
        }
    }

    pub fn switch_all(&self, action_type: ActionType) {
        let map = self.devices.read().unwrap();
        map.iter()
            .for_each(|(_, d)| d.switch(&action_type));
    }

    pub fn for_each<A>(&self, action: A)
        where A: Fn(&DeviceBox) {
        let map = self.devices.read().unwrap();
        map.iter().for_each(|(_, d)| action(d));
    }

    pub fn set_power(&self, id: &str, power: u8) {
        let map = self.devices.read().unwrap();
        if let Some(device) = map.get(id) {
            device.set_power(power)
        }
    }

    pub fn set_state(&self, id: &str, action_type: ActionType, power: u8) {
        let map = self.devices.read().unwrap();
        if let Some(device) = map.get(id) {
            device.set_state(&action_type, power);
        }
    }
}

unsafe impl Sync for DeviceBox {}

unsafe impl Send for DeviceBox {}

impl Add<SerialDimmer> for DeviceHandler {
    type Output = DeviceHandler;

    fn add(self, device: SerialDimmer) -> DeviceHandler {
        {
            let mut map = self.devices.write().unwrap();
            map.insert(device.id().to_string(), DeviceBox::SerialDimmer((device, timer())));
        }
        self
    }
}

impl Add<WebDimmer> for DeviceHandler {
    type Output = DeviceHandler;

    fn add(self, device: WebDimmer) -> DeviceHandler {
        {
            let mut map = self.devices.write().unwrap();
            map.insert(device.id().to_string(), DeviceBox::WebDimmer((device, timer())));
        }
        self
    }
}

impl Add<WebLed> for DeviceHandler {
    type Output = DeviceHandler;

    fn add(self, device: WebLed) -> DeviceHandler {
        {
            let mut map = self.devices.write().unwrap();
            map.insert(device.id().to_string(), DeviceBox::WebLed((device, timer())));
        }
        self
    }
}

impl AddAssign<SerialDimmer> for DeviceHandler {
    fn add_assign(&mut self, device: SerialDimmer) {
        let mut map = self.devices.write().unwrap();
        map.insert(device.id().to_string(), DeviceBox::SerialDimmer((device, timer())));
    }
}

impl AddAssign<WebDimmer> for DeviceHandler {
    fn add_assign(&mut self, device: WebDimmer) {
        let mut map = self.devices.write().unwrap();
        map.insert(device.id().to_string(), DeviceBox::WebDimmer((device, timer())));
    }
}

impl AddAssign<WebLed> for DeviceHandler {
    fn add_assign(&mut self, device: WebLed) {
        let mut map = self.devices.write().unwrap();
        map.insert(device.id().to_string(), DeviceBox::WebLed((device, timer())));
    }
}

impl SubAssign<&str> for DeviceHandler {
    fn sub_assign(&mut self, rhs: &str) {
        let mut devices = self.devices.write().unwrap();
        devices.remove(rhs);
    }
}

impl Debug for DeviceHandler {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "DeviceHandler:{:?}", self.devices.read().unwrap())
    }
}