use crate::home::automation::web_beam_updater::BgHolder;
use crate::home::automation::BackgroundBuilder;
use crate::home::configuration::{Configuration, State};
use crate::home::Home;
use crate::io::IO;
use crate::runtime::Background;
use anyhow::Error;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;

const WEB_UPDATER: &str = "blackout";

const INTERVAL: Duration = Duration::from_secs(20 * 60);

pub struct Blackout {
    config: Configuration,
    home: Home,
    io: IO,
    state: Arc<RwLock<Config>>,
}

impl Blackout {
    pub fn new(home: &Home, io: &IO, config: &Configuration) -> Result<Blackout, Error> {
        let state = Config::default();
        config.reg(WEB_UPDATER, state.clone());

        Ok(Blackout {
            config: config.clone(),
            home: home.clone(),
            io: io.clone(),
            state: Arc::new(RwLock::new(state)),
        })
    }

    fn auth_shutdown(&self) {}
}

impl BackgroundBuilder for Blackout {
    fn build(self) -> Background {
        let rt = self.io.runtime();
        Background::every(rt, INTERVAL.clone(), true, move || self.auth_shutdown())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    device: Arc<RwLock<HashMap<String, Policy>>>,
}

impl State for Config {
    fn init(&self, value: Option<Value>) -> Result<Option<Value>, Error> {
        match value {
            Some(val) => {
                let val: HashMap<String, Policy> = serde_json::from_value(
                    val.get("device")
                        .ok_or_else(|| Error::msg("Expected device field"))?
                        .clone()
                )?;
                *self.device.write().unwrap() = val;
                Ok(None)
            }
            None => {
                let default = default();
                let value = serde_json::to_value(&default)?;
                *self.device.write().unwrap() = default;
                Ok(Some(value))
            }
        }
    }

    fn update(&self, value: Value) -> Result<Value, Error> {
        let val: HashMap<String, Policy> = serde_json::from_value(
            val.get("device")
                .ok_or_else(|| Error::msg("Expected device field"))?
                .clone()
        )?;
        *self.device.write().unwrap() = val;

        Ok(value)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Policy {
    None,
    Shutdown(Duration),
}

fn default() -> HashMap<String, Policy> {
    let mut map = HashMap::new();

    map
}
