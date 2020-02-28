use devices::{SerialDimmer, Switch as SwitchTrait, WebBeam};
use home::Home;
use io::IOBuilder;
use sensors::Switch;
use anyhow::Result;

#[derive(Debug)]
pub struct Kitchen {
    pub beam: WebBeam,
    pub kitchen_lamp: SerialDimmer,
    pub switch_1: Switch,
    pub switch_2: Switch,
}

impl Kitchen {
    pub fn new(io: &mut IOBuilder) -> Kitchen {
        Kitchen {
            beam: WebBeam::new(io, "kitchen_beam"),
            kitchen_lamp: SerialDimmer::new(io, "kitchen_lamp", 0x04, 1, 100),
            switch_1: Switch::new(io, "kitchen_1", Kitchen::on_kitchen_switch_1),
            switch_2: Switch::new(io, "kitchen_2", Kitchen::on_kitchen_switch_2),
        }
    }

    fn on_kitchen_switch_1(home: &Home, is_on: bool) -> Result<()> {
        home.kitchen.kitchen_lamp.set_power(100);
        home.kitchen.kitchen_lamp.switch(is_on);
        Ok(())
    }

    fn on_kitchen_switch_2(home: &Home, is_on: bool) -> Result<()> {
        home.kitchen.beam.switch(is_on);
        Ok(())
    }
}
