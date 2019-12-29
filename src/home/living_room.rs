use devices::{SerialSwitch, Switch as SwitchTrait, WebBeam};
use home::Home;
use io::IOBuilder;
use sensors::Switch;

#[derive(Debug)]
pub struct LivingRoom {
    //main light
    pub chandelier: SerialSwitch,
    //light on the treadmill
    pub cupboard_lamp: SerialSwitch,
    // 2 channel beam.
    pub beam: WebBeam,
    pub switch_1: Switch,
    pub switch_2: Switch,
}

impl LivingRoom {
    pub fn new(io: &mut IOBuilder) -> LivingRoom {
        LivingRoom {
            chandelier: SerialSwitch::new(io, "living_room_lamp", 0x02),
            cupboard_lamp: SerialSwitch::new(io, "cupboard_lamp", 0x06),
            beam: WebBeam::new(io, "lounge_beam"),
            switch_1: Switch::new(io, "lounge_1", LivingRoom::on_switch_1),
            switch_2: Switch::new(io, "lounge_2", LivingRoom::on_switch_2),
        }
    }

    //beam switch
    fn on_switch_1(home: &Home, is_on: bool) -> Result<(), String> {
        home.living_room.beam.switch(is_on);
        Ok(())
    }

    fn on_switch_2(home: &Home, is_on: bool) -> Result<(), String> {
        home.living_room.chandelier.switch(is_on);
        Ok(())
    }
}
