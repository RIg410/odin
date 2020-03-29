use crate::devices::DeviceType;
use crate::home::automation::BackgroundBuilder;
use crate::home::configuration::{ConfigValue, Configuration, OnUpdate};
use crate::io::IO;
use crate::log_error;
use crate::runtime::{Background, Runtime};
use anyhow::Error;
use serde_json::Value;
use std::time::Duration;

const WEB_UPDATER: &str = "web_updater";

const INTERVAL: Duration = Duration::from_secs(20);

pub struct WebBeamUpdater {
    io: IO,
    bg: BgHolder,
}

impl WebBeamUpdater {
    pub fn new(io: &IO, config: &Configuration) -> Result<WebBeamUpdater, Error> {
        let holder = BgHolder::default();
        config.add(
            WEB_UPDATER,
            ConfigValue::new(Config { interval: INTERVAL.clone() }, holder.clone())?,
        );
        Ok(WebBeamUpdater {
            io: io.clone(),
            bg: holder,
        })
    }
}

impl BackgroundBuilder for WebBeamUpdater {
    fn build(&mut self) -> Background {
        self.bg.run(&self.io.runtime(), &self.io)
    }
}

fn update_web_devices(io: &IO) {
    io.device_holder()
        .devices()
        .iter()
        .for_each(|(_, device)| match device.dev_type() {
            DeviceType::WebBeam => {
                log_error!(&device.flush());
            }
            _ => {}
        });
}

#[derive(Default, Clone, Debug)]
pub struct BgHolder {
    bg: Option<Background>,
}

impl BgHolder {
    pub fn run(&mut self, rt: &Runtime, io: &IO) -> Background {
        let io = io.clone();
        let bg = Background::every(rt, INTERVAL.clone(), true, move || update_web_devices(&io));
        self.bg = Some(bg.clone());
        bg
    }
}

impl OnUpdate for BgHolder {
    fn on_update(&self, value: Value) -> Result<(), Error> {
        let config: Config = serde_json::from_value(value)?;
        if let Some(bg) = &self.bg {
            info!("Update web updater interval: {:?}", config);
            bg.update_interval(config.interval);
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    interval: Duration
}