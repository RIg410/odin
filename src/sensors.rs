use std::borrow::ToOwned;
use std::string::ToString;
use home::Home;
use std::fmt::{Debug, Formatter, Error};
use std::sync::atomic::{AtomicBool, Ordering};

pub type Action = Fn(&Home, ActionType) + Sync + Send + 'static;

pub trait OnSwitch {
    fn on(&self, home: &Home);
    fn off(&self, home: &Home);
    fn toggle(&self, home: &Home);
}

pub struct Switch {
    id: String,
    action: Box<Action>,
    is_on: AtomicBool,
}

impl Switch {
    pub fn new<A>(id: &str, act: A) -> Switch
        where A: Fn(&Home, ActionType) + Sync + Send + 'static {
        Switch {
            id: id.to_string(),
            action: Box::new(act),
            is_on: AtomicBool::new(false),
        }
    }

    fn action(&self, home: &Home) {
        let action_type = if self.is_on.load(Ordering::SeqCst) {
            ActionType::On
        } else {
            ActionType::Off
        };

        (self.action)(home, action_type);
    }
}

impl OnSwitch for Switch {
    fn on(&self, home: &Home) {
        self.is_on.store(true, Ordering::SeqCst);
        self.action(home);
    }

    fn off(&self, home: &Home) {
        self.is_on.store(false, Ordering::SeqCst);
        self.action(home);
    }

    fn toggle(&self, home: &Home) {
        self.is_on.store(self.is_on.load(Ordering::SeqCst), Ordering::SeqCst);
        self.action(home);
    }
}

pub enum ActionType {
    On,
    Off,
}

impl Debug for Switch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Switch {{ {} }}", self.id)
    }
}