use controller::{Lighting, DeviceHolder};
use std::sync::Arc;
use super::MessageHandler;
use super::super::transport::{MqPublisher, Message};
use super::*;
use super::super::configuration::SwitchConfiguration as Config;

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
        let body = msg.payload_as_string().map_err(|err| Some(err))?;

        let v: Vec<_> = lamp_ids.iter()
            .map(|id|{self.device_holder.get(id)})
            .collect();

//        match body {
//            "off" => {}
//            "on" => {}
//            "toggle" => {}
//        }

        for id in lamp_ids {}
        Ok(None)
    }
}
