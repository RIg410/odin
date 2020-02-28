use devices::{SerialSwitch, Switch as SwitchTrait};
use home::Home;
use io::IOBuilder;
use sensors::Switch;
use anyhow::Result;

#[derive(Debug)]
pub struct Balcony {
    //main light
    pub chandelier: SerialSwitch,
    pub switch_1: Switch,
    pub switch_2: Switch,
}

impl Balcony {
    pub fn new(io: &mut IOBuilder) -> Balcony {
        Balcony {
            chandelier: SerialSwitch::new(io, "balcony_lamp", 0x05),
            switch_1: Switch::new(io, "balcony_1", Balcony::on_balcony_switch_1),
            switch_2: Switch::new(io, "balcony_2", Balcony::on_balcony_switch_2),
        }
    }

    fn on_balcony_switch_1(home: &Home, is_on: bool) -> Result<()> {
        home.balcony.chandelier.switch(is_on);
        Ok(())
    }

    fn on_balcony_switch_2(home: &Home, is_on: bool) -> Result<()> {
        let lamp = &home.kitchen.kitchen_lamp;
        lamp.set_power(1);
        lamp.switch(is_on);
        Ok(())
    }
}
