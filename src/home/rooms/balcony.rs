use crate::devices::{SerialSwitch, Switch as SwitchTrait};
use crate::home::Home;
use crate::io::IOMut;
use crate::sensors::Switch;
use anyhow::Result;

#[derive(Debug)]
pub struct Balcony {
    pub lamp: SerialSwitch,
    pub switch_1: Switch,
    pub switch_2: Switch,
}

impl Balcony {
    pub fn new(io: &mut IOMut) -> Balcony {
        Balcony {
            lamp: SerialSwitch::new(io, "balcony_lamp", 0x05),
            switch_1: Switch::toggle(io, "balcony_1", Balcony::on_balcony_switch_1),
            switch_2: Switch::toggle(io, "balcony_2", Balcony::on_balcony_switch_2),
        }
    }

    fn on_balcony_switch_1(home: &Home) -> Result<()> {
        home.balcony.lamp.toggle()
    }

    fn on_balcony_switch_2(home: &Home) -> Result<()> {
        let lamp = &home.kitchen.kitchen_lamp;
        lamp.set_power(1);
        lamp.toggle()
    }
}
