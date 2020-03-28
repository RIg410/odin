mod scheme;
mod switch;

use serde_json::Value;
use crate::home::Home;
use std::fmt::{Debug, Formatter};
use std::collections::HashMap;
use anyhow::Result;
use crate::home::scripts::scheme::scheme_scripts;
use crate::home::scripts::switch::switch_scripts;
use std::fmt::Error as FmtError;
pub use switch::SWITCH_OFF_ALL;

pub trait Runner {
    fn run_script(&self, name: &str, value: Value) -> Result<()>;
}

pub struct Script {
    inner: Box<dyn Fn(&Home, Value) -> Result<()> + Send + Sync + 'static>,
}

impl Script {
    fn new<A>(act: A) -> Script
        where
            A: Fn(&Home, Value) -> Result<()> + Send + Sync + 'static,
    {
        Script {
            inner: Box::new(act),
        }
    }

    pub fn run(&self, home: &Home, value: Value) -> Result<()> {
        (self.inner)(home, value)
    }
}

impl Debug for Script {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        write!(f, "action")
    }
}

pub fn scripts() -> HashMap<String, Script> {
    let mut map = HashMap::new();
    scheme_scripts(&mut map);
    switch_scripts(&mut map);
    map
}

