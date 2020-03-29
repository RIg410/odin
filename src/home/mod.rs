mod automation;
pub mod configuration;
mod rooms;
pub(crate) mod scripts;

use crate::home::configuration::Configuration;
use crate::home::rooms::bad_room::BadRoom;
use crate::home::rooms::balcony::Balcony;
use crate::home::rooms::bathroom::Bathroom;
use crate::home::rooms::corridor::Corridor;
use crate::home::rooms::kitchen::Kitchen;
use crate::home::rooms::living_room::LivingRoom;
use crate::home::rooms::toilet::Toilet;
use crate::home::scripts::{Runner, Script};
use crate::io::IOMut;
use anyhow::{Error, Result};
pub use automation::BackgroundProcess;
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};

#[derive(Debug, Clone)]
pub struct Home {
    pub bad_room: Arc<BadRoom>,
    pub living_room: Arc<LivingRoom>,
    pub kitchen: Arc<Kitchen>,
    pub balcony: Arc<Balcony>,
    pub corridor: Arc<Corridor>,
    pub toilet: Arc<Toilet>,
    pub bathroom: Arc<Bathroom>,
    pub scripts: Arc<HashMap<String, Script>>,
}

impl Home {
    pub fn new(io: &mut IOMut, config: &Configuration) -> Home {
        let home = Home {
            bad_room: Arc::new(BadRoom::new(io)),
            living_room: Arc::new(LivingRoom::new(io)),
            kitchen: Arc::new(Kitchen::new(io)),
            balcony: Arc::new(Balcony::new(io)),
            corridor: Arc::new(Corridor::new(io)),
            toilet: Arc::new(Toilet::new(io)),
            bathroom: Arc::new(Bathroom::new(io)),
            scripts: Arc::new(scripts::scripts()),
        };

        home
    }
}

impl Runner for Home {
    fn run_script(&self, name: &str, value: Value) -> Result<()> {
        self.scripts
            .get(name)
            .ok_or_else(|| Error::msg(format!("Unknown script: {}", name)))
            .and_then(|script| script.run(self, value))
    }
}
