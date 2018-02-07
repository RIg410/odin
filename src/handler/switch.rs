use controller::{Lighting, DeviceHolder};
use std::sync::Arc;
use std::collections::HashMap;
use controller::Device;
use super::MessageHandler;
use transport::{MqPublisher, Message};
use super::*;
use configuration::SwitchConfiguration as Config;

pub struct Switch {
    device_holder: Arc<DeviceHolder>,
    config: Arc<Config>,
}

impl Switch {
    pub fn new(device_holder: Arc<DeviceHolder>, config: Arc<Config>) -> Switch {
        Switch { device_holder, config }
    }

    fn get_light_id(&self, topic: &str) -> Result<&Vec<String>, Option<String>> {
        let switch_id = parse_sender(topic);
        if let Some(id) = switch_id {
            let ids = self.config.get_lights_ids(id);
            if let Some(l_ids) = ids {
                Ok(l_ids)
            } else {
                Err(Some(format!("There are not lights for switch with id :{}", id)))
            }
        } else {
            Err(Some("Failed to parse topic".to_owned()))
        }
    }
}

impl MessageHandler for Switch {
    fn handel(&self, msg: &Message, publisher: &mut MqPublisher) -> Result<Option<String>, Option<String>> {
        let lamp_ids = self.get_light_id(msg.topic)?;
        let action = msg.payload[0];

        if action < 0x01 || action > 0x03 {
            return Err(Some(format!("Unsupported action: [{}]", action)));
        }

        let devices: HashMap<&String, Option<&Box<Device>>> = lamp_ids.iter()
            .map(|id| { (id, self.device_holder.get(id)) })
            .collect();

        let mut err = String::new();
        for dev in devices {
            if !dev.1.is_some() {
                err.push_str(&format!("Device with id = {} not fount;", dev.0));
                continue;
            }

            let dev = dev.1.unwrap();
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
