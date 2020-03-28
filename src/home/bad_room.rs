use crate::devices::{SerialSwitch, Switch as SwitchTrait};
use crate::home::Home;
use crate::io::IOMut;
use crate::sensors::Switch;
use anyhow::Result;
use crate::home::scripts::{Runner, SWITCH_OFF_ALL};
use serde_json::Value;

#[derive(Debug)]
pub struct BadRoom {
    pub chandelier: SerialSwitch,
    pub switch_1: Switch,
    pub switch_2: Switch,
}

impl BadRoom {
    pub fn new(io: &mut IOMut) -> BadRoom {
        BadRoom {
            chandelier: SerialSwitch::new(io, "bedroom_lamp", 0x01),
            switch_1: Switch::new(io, "bedroom_1", BadRoom::on_switch_1),
            switch_2: Switch::new(io, "bedroom_2", BadRoom::on_switch_2),
        }
    }

    fn on_switch_1(home: &Home, _is_on: bool) -> Result<()> {
        home.run_script(SWITCH_OFF_ALL, Value::Null)
    }

    fn on_switch_2(home: &Home, is_on: bool) -> Result<()> {
        home.bad_room.chandelier.switch(is_on)
    }
}
