use std::borrow::ToOwned;
use std::string::ToString;
use home::Home;
use std::fmt::{Debug, Formatter, Error};

pub type Action = Fn(&Home, ActionType) + Sync + Send + 'static;

pub struct Switch {
    id: String,
    action: Box<Action>,
}

impl Switch {
    pub fn new<A>(id: &str, act: A) -> Switch
        where A: Fn(&Home, ActionType) + Sync + Send + 'static {
        Switch {
            id: id.to_string(),
            action: Box::new(act),
        }
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