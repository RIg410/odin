mod bad_room;
mod balcony;
mod bathroom;
mod corridor;
mod kitchen;
mod living_room;
mod script;
mod toilet;

pub use home::script::Runner;
use std::{collections::HashMap, sync::Arc};

use home::script::Script;
use home::{
    bad_room::BadRoom, balcony::Balcony, bathroom::Bathroom, corridor::Corridor, kitchen::Kitchen,
    living_room::LivingRoom, toilet::Toilet,
};
use io::IOBuilder;
use serde_json::Value;
use anyhow::{
    Result, Error
};

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
    pub fn new(io: &mut IOBuilder) -> Home {
        Home {
            bad_room: Arc::new(BadRoom::new(io)),
            living_room: Arc::new(LivingRoom::new(io)),
            kitchen: Arc::new(Kitchen::new(io)),
            balcony: Arc::new(Balcony::new(io)),
            corridor: Arc::new(Corridor::new(io)),
            toilet: Arc::new(Toilet::new(io)),
            bathroom: Arc::new(Bathroom::new(io)),
            scripts: Arc::new(script::scripts_map()),
        }
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
