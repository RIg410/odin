use std::collections::HashMap;
use crate::io::IO;
use crate::home::automation::web_beam_updater::BgHolder;
use crate::runtime::Background;
use crate::home::automation::BackgroundBuilder;
use std::time::Duration;
use crate::home::configuration::Configuration;
use crate::home::Home;
use anyhow::Error;

const WEB_UPDATER: &str = "auto_shutdown";

const INTERVAL: Duration = Duration::from_secs(20 * 60);

pub struct AutoShutdown {
    home: Home,
    io: IO,
}

impl AutoShutdown {
    pub fn new(home: &Home, io: &IO, config: &Configuration) -> Result<AutoShutdown, Error> {
        Ok(AutoShutdown {
            home: home.clone(),
            io: io.clone(),
        })
    }
}

impl BackgroundBuilder for AutoShutdown {
    fn build(&mut self) -> Background {
        let rt = self.io.runtime();
        let home = self.home.clone();
        Background::every(rt, INTERVAL.clone(), true, move || auth_shutdown(&home))
    }
}

fn auth_shutdown(io: &Home) {
   //io.device_holder().
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    device: HashMap<String, Policy>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Policy {
    None,
    Shutdown(Duration),
}