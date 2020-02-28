use devices::{SerialDimmer, SerialSwitch, Switch as SwitchTrait, WebSwitch};
use home::Home;
use io::IOBuilder;
use sensors::Switch;
use anyhow::Result;

#[derive(Debug)]
pub struct Bathroom {
    pub lamp: SerialDimmer,
    pub fun: SerialSwitch,
    pub hot_water: WebSwitch,
    pub cold_water: WebSwitch,
    pub return_water: WebSwitch,
    pub switch: Switch,
}

impl Bathroom {
    pub fn new(io: &mut IOBuilder) -> Bathroom {
        let lamp = SerialDimmer::new(io, "bedroom_lamp", 0x01, 20, 100);
        lamp.switch(false);
        Bathroom {
            lamp,
            fun: SerialSwitch::new(io, "bathroom_fun", 0x04),
            hot_water: WebSwitch::new(io, "hot_water"),
            cold_water: WebSwitch::new(io, "cold_water"),
            return_water: WebSwitch::new(io, "return_water"),
            switch: Switch::new(io, "bathroom", Bathroom::on_switch),
        }
    }

    fn on_switch(home: &Home, is_on: bool) -> Result<()> {
        home.bathroom.lamp.switch(is_on);
        Ok(())
    }
}
