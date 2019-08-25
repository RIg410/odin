mod bad_room;
mod script;
mod living_room;
mod kitchen;
mod balcony;
mod corridor;
mod toilet;
mod bathroom;

use std::{
    sync::Arc,
    collections::HashMap,
};
pub use home::script::Runner;

use home::{
    bad_room::BadRoom,
    living_room::LivingRoom,
    kitchen::Kitchen,
    balcony::Balcony,
    corridor::Corridor,
    toilet::Toilet,
    bathroom::Bathroom,
};
use io::IOBuilder;
use serde_json::Value;
use home::script::Script;

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
    fn run_script(&self, name: &str, value: Value) -> Result<(), String> {
        self.scripts.get(name)
            .ok_or_else(|| format!("Unknown script: {}", name))
            .and_then(|script| script.run(self, value))
    }
}