use controller::Switch;
use std::sync::Arc;
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

    #[inline]
    fn get_switch(&self, topic: &str) -> Result<&Switch, Option<String>> {
        let switch_id = parse_id(topic);
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

        match String::from_utf8_lossy(&msg.payload).as_ref() {
            "ON" => {
                switch.switch_on(publisher);
            }
            "OFF" => {
                switch.switch_off(publisher);
            }
            a @ _ => {
                return Err(Some(format!("Unsupported action: [{}]", a)));
            }
        }
        Ok(None)
    }
}
