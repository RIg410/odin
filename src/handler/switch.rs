use controller::{Lighting, DeviceHolder, Switch};
use std::sync::Arc;
use std::collections::HashMap;
use controller::Device;
use super::MessageHandler;
use transport::{MqPublisher, Message};
use super::*;
use configuration::SwitchConfiguration as Config;

pub struct SwitchHandler {
    config: Arc<Config>,
}

impl SwitchHandler {
    pub fn new(config: Arc<Config>) -> SwitchHandler {
        SwitchHandler { config }
    }

    fn get_switch(&self, topic: &str) -> Result<&Switch, Option<String>> {
        let switch_id = parse_sender(topic);
        if let Some(id) = switch_id {
            let switch = self.config.get_switch(id);
            if let Some(switch) = switch {
                Ok(switch)
            } else {
                Err(Some(format!("There are not lights for switch with id :{}", id)))
            }
        } else {
            Err(Some("Failed to parse topic".to_owned()))
        }
    }
}

impl MessageHandler for SwitchHandler {
    fn handel(&self, msg: &Message, publisher: &mut MqPublisher) -> Result<Option<String>, Option<String>> {
        let switch = self.get_switch(msg.topic)?;
        let action = msg.payload[0];

        if action < 0x01 || action > 0x03 {
            return Err(Some(format!("Unsupported action: [{}]", action)));
        }

        let mut err = String::new();
        for dev in &switch.devices {
            match action {
                0x01 /*on*/ => {
                    if let Err(why) = dev.on() {
                        err.push_str(&format!("Fail to on device {:?}[{:?}];", dev, why));
                        continue;
                    }
                }
                0x02 /*off*/ => {
                    if let Err(why) = dev.off() {
                        err.push_str(&format!("Fail to off device {:?}[{:?}];", dev, why));
                        continue;
                    }
                }
                0x03 /*toggle*/ => {
                    if let Err(why) = dev.toggle() {
                        err.push_str(&format!("Fail to toggle device {:?}[{:?}];", dev, why));
                        continue;
                    }
                }
                a @ _ => {
                    err.push_str(&format!("Unsupported action: [{}]", a));
                    continue;
                }
            }
            match dev.flush(publisher) {
                Err(why) => {
                    err.push_str(&format!("Fail to flush device [{:?}], err=[{:?}]", dev, why));
                }
                Ok(_) => {}
            }
        }

        if err.is_empty() {
            Ok(None)
        } else {
            Err(Some(err))
        }
    }
}
