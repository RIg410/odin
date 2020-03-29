use crate::home::automation::BackgroundBuilder;
use crate::runtime::Background;
use crate::io::IO;
use crate::home::Home;
use std::time::Duration;
use crate::devices::DeviceType;
use crate::log_error;
use crate::home::configuration::Configuration;

pub struct WebBeamUpdater();

impl BackgroundBuilder for WebBeamUpdater {
    fn build(&self, _home: &Home, io: &IO, config: &Configuration) -> Background {
        let rt = io.runtime();
        let io = io.clone();
        Background::every(rt, Duration::from_secs(20), true, move || { update_web_devices(&io) })
    }
}

fn update_web_devices(io: &IO) {
    io.device_holder().devices().iter()
        .for_each(|(_, device)| {
            match device.dev_type() {
                DeviceType::WebBeam => {
                    log_error!(&device.flush());
                }
                _ => {}
            }
        });
}