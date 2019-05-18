use io::IOBuilder;
use devices::{SerialSwitch, WebBeam, SerialDimmer, WebSwitch};
use sensors::Switch;

#[derive(Debug)]
pub struct Home {
    pub bad_room: BadRoom,
    pub living_room: LivingRoom,
    pub kitchen: Kitchen,
    pub balcony: Balcony,
    pub corridor: Corridor,
    pub toilet: Toilet,
    pub bathroom: Bathroom,
}

impl Home {
    pub fn new(io: &mut IOBuilder) -> Home {
        Home {
            bad_room: BadRoom::new(io),
            living_room: LivingRoom::new(io),
            kitchen: Kitchen::new(io),
            balcony: Balcony::new(io),
            corridor: Corridor::new(io),
            toilet: Toilet::new(io),
            bathroom: Bathroom::new(io),
        }
    }
}

///Living room controller.
#[derive(Debug)]
pub struct LivingRoom {
    //main light
    chandelier: SerialSwitch,
    //light on the treadmill
    cupboard_lamp: SerialSwitch,
    // 2 channel beam.
    beam: WebBeam,
    switch_1: Switch,
    switch_2: Switch,
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
    fn on_switch_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }

    fn on_switch_2(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}

///Kitchen room controller.
#[derive(Debug)]
pub struct Kitchen {
    beam: WebBeam,
    kitchen_lamp: SerialDimmer,
    switch_1: Switch,
    switch_2: Switch,
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

    fn on_kitchen_switch_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }

    fn on_kitchen_switch_2(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}

///Balcony room controller.
#[derive(Debug)]
pub struct Balcony {
    //main light
    chandelier: SerialSwitch,
    switch_1: Switch,
    switch_2: Switch,
}

impl Balcony {
    pub fn new(io: &mut IOBuilder) -> Balcony {
        Balcony {
            chandelier: SerialSwitch::new(io, "balcony_lamp", 0x05),
            switch_1: Switch::new(io, "balcony_1", Balcony::on_balcony_switch_1),
            switch_2: Switch::new(io, "balcony_2", Balcony::on_balcony_switch_2),
        }
    }

    fn on_balcony_switch_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }

    fn on_balcony_switch_2(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Corridor {
    lamp: SerialDimmer,
    beam: WebBeam,
    exit_1: Switch,
    exit_2: Switch,
    ir_sensor_front_door: Switch,
    ir_sensor_bedroom_door: Switch,
    ir_sensor_middle: Switch,
    ir_sensor_middle_1: Switch,
    ir_sensor_living_room: Switch,
    ir_sensor_living_room_1: Switch,
}

impl Corridor {
    pub fn new(io: &mut IOBuilder) -> Corridor {
        Corridor {
            lamp: SerialDimmer::new(io, "corridor_lamp", 0x03, 1, 100),
            beam: WebBeam::new(io, "corridor_beam"),
            exit_1: Switch::new(io, "exit_1", Corridor::on_exit_1),
            exit_2: Switch::new(io, "exit_2", Corridor::on_exit_2),
            ir_sensor_front_door: Switch::new(io, "ir_sensor_front_door", Corridor::ir_sensor_front_door),
            ir_sensor_bedroom_door: Switch::new(io, "ir_sensor_bedroom_door", Corridor::ir_sensor_bedroom_door),
            ir_sensor_middle: Switch::new(io, "ir_sensor_middle", Corridor::ir_sensor_middle),
            ir_sensor_middle_1: Switch::new(io, "ir_sensor_middle_1", Corridor::ir_sensor_middle_1),
            ir_sensor_living_room: Switch::new(io, "ir_sensor_living_room", Corridor::ir_sensor_living_room),
            ir_sensor_living_room_1: Switch::new(io, "ir_sensor_living_room_1", Corridor::ir_sensor_living_room_1),
        }
    }

    fn on_exit_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn on_exit_2(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn ir_sensor_front_door(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn ir_sensor_bedroom_door(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn ir_sensor_middle(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn ir_sensor_middle_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn ir_sensor_living_room(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
    fn ir_sensor_living_room_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Toilet {
    lamp: SerialDimmer,
    fun: SerialSwitch,
    switch: Switch,
}

impl Toilet {
    pub fn new(io: &mut IOBuilder) -> Toilet {
        Toilet {
            lamp: SerialDimmer::new(io, "toilet_lamp", 0x02, 25, 100),
            fun: SerialSwitch::new(io, "toilet_fun", 0x03),
            switch: Switch::new(io, "toilet", Toilet::on_switch),
        }
    }

    fn on_switch(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Bathroom {
    lamp: SerialDimmer,
    fun: SerialSwitch,
    hot_water: WebSwitch,
    cold_water: WebSwitch,
    return_water: WebSwitch,
    switch: Switch,
}

impl Bathroom {
    pub fn new(io: &mut IOBuilder) -> Bathroom {
        Bathroom {
            lamp: SerialDimmer::new(io, "bedroom_lamp", 0x01, 20, 100),
            fun: SerialSwitch::new(io, "bathroom_fun", 0x04),
            hot_water: WebSwitch::new(io, "hot_water"),
            cold_water: WebSwitch::new(io, "cold_water"),
            return_water: WebSwitch::new(io, "return_water"),
            switch: Switch::new(io, "toilet", Bathroom::on_switch),
        }
    }

    fn on_switch(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct BadRoom {
    chandelier: SerialSwitch,
    beam: WebBeam,
    switch_1: Switch,
    switch_2: Switch,
}

impl BadRoom {
    pub fn new(io: &mut IOBuilder) -> BadRoom {
        BadRoom {
            chandelier: SerialSwitch::new(io, "bedroom_lamp", 0x01),
            beam: WebBeam::new(io, "bedroom_beam"),
            switch_1: Switch::new(io, "bedroom_1", BadRoom::on_switch_1),
            switch_2: Switch::new(io, "bedroom_1", BadRoom::on_switch_2),
        }
    }

    fn on_switch_1(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }

    fn on_switch_2(this: &Home, is_on: bool) -> Result<(), String> {
        Ok(())
    }
}
