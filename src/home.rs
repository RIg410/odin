use transport::Transport;
use devices::{SerialSwitch, WebBeam, SerialDimmer};
use sensors::{Switch, ActionType};

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
    pub fn new(tr: &Transport) -> Home {
        Home {
            bad_room: BadRoom::new(tr),
            living_room: LivingRoom::new(tr),
            kitchen: Kitchen::new(tr),
            balcony: Balcony::new(tr),
            corridor: Corridor::new(tr),
            toilet: Toilet::new(tr),
            bathroom: Bathroom::new(tr),
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
    pub fn new(tr: &Transport) -> LivingRoom {
        LivingRoom {
            chandelier: SerialSwitch::new(tr, "living_room_lamp", 0x02),
            cupboard_lamp: SerialSwitch::new(tr, "cupboard_lamp", 0x06),
            beam: WebBeam::new(tr, "lounge_beam"),
            switch_1: Switch::new("lounge_1", LivingRoom::on_switch_1),
            switch_2: Switch::new("lounge_2", LivingRoom::on_switch_2),
        }
    }

    fn on_switch_1(this: &Home, action_type: ActionType) {}

    fn on_switch_2(this: &Home, action_type: ActionType) {}
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
    pub fn new(tr: &Transport) -> Kitchen {
        Kitchen {
            beam: WebBeam::new(tr, "kitchen_beam"),
            kitchen_lamp: SerialDimmer::new(tr, "kitchen_lamp", 0x04, 1, 100),
            switch_1: Switch::new("kitchen_1", Kitchen::on_kitchen_switch_1),
            switch_2: Switch::new("kitchen_2", Kitchen::on_kitchen_switch_2),
        }
    }

    fn on_kitchen_switch_1(this: &Home, action_type: ActionType) {}

    fn on_kitchen_switch_2(this: &Home, action_type: ActionType) {}
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
    pub fn new(tr: &Transport) -> Balcony {
        Balcony {
            chandelier: SerialSwitch::new(tr, "balcony_lamp", 0x05),
            switch_1: Switch::new("balcony_1", Balcony::on_balcony_switch_1),
            switch_2: Switch::new("balcony_2", Balcony::on_balcony_switch_2),
        }
    }

    fn on_balcony_switch_1(this: &Home, action_type: ActionType) {}

    fn on_balcony_switch_2(this: &Home, action_type: ActionType) {}
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
    pub fn new(tr: &Transport) -> Corridor {
        Corridor {
            lamp: SerialDimmer::new(tr, "corridor_lamp", 0x03, 1, 100),
            beam: WebBeam::new(tr, "corridor_beam"),
            exit_1: Switch::new("exit_1", Corridor::on_exit_1),
            exit_2: Switch::new("exit_2", Corridor::on_exit_2),
            ir_sensor_front_door: Switch::new("ir_sensor_front_door", Corridor::ir_sensor_front_door),
            ir_sensor_bedroom_door: Switch::new("ir_sensor_bedroom_door", Corridor::ir_sensor_bedroom_door),
            ir_sensor_middle: Switch::new("ir_sensor_middle", Corridor::ir_sensor_middle),
            ir_sensor_middle_1: Switch::new("ir_sensor_middle_1", Corridor::ir_sensor_middle_1),
            ir_sensor_living_room: Switch::new("ir_sensor_living_room", Corridor::ir_sensor_living_room),
            ir_sensor_living_room_1: Switch::new("ir_sensor_living_room_1", Corridor::ir_sensor_living_room_1),
        }
    }

    fn on_exit_1(this: &Home, action_type: ActionType) {}
    fn on_exit_2(this: &Home, action_type: ActionType) {}
    fn ir_sensor_front_door(this: &Home, action_type: ActionType) {}
    fn ir_sensor_bedroom_door(this: &Home, action_type: ActionType) {}
    fn ir_sensor_middle(this: &Home, action_type: ActionType) {}
    fn ir_sensor_middle_1(this: &Home, action_type: ActionType) {}
    fn ir_sensor_living_room(this: &Home, action_type: ActionType) {}
    fn ir_sensor_living_room_1(this: &Home, action_type: ActionType) {}
}

#[derive(Debug)]
pub struct Toilet {
    lamp: SerialDimmer,
    fun: SerialSwitch,
    switch: Switch,
}

impl Toilet {
    pub fn new(tr: &Transport) -> Toilet {
        Toilet {
            lamp: SerialDimmer::new(tr, "toilet_lamp", 0x02, 25, 100),
            fun: SerialSwitch::new(tr, "toilet_fun", 0x03),
            switch: Switch::new("toilet", Toilet::on_switch),
        }
    }

    fn on_switch(this: &Home, action_type: ActionType) {}
}


#[derive(Debug)]
pub struct Bathroom {
    lamp: SerialDimmer,
    fun: SerialSwitch,
    switch: Switch,
}

impl Bathroom {
    pub fn new(tr: &Transport) -> Bathroom {
        Bathroom {
            lamp: SerialDimmer::new(tr, "bedroom_lamp", 0x01, 20, 100),
            fun: SerialSwitch::new(tr, "bathroom_fun", 0x04),
            switch: Switch::new("toilet", Bathroom::on_switch),
        }
    }

    fn on_switch(this: &Home, action_type: ActionType) {}
}


#[derive(Debug)]
pub struct BadRoom {
    chandelier: SerialSwitch,
    beam: WebBeam,
    switch_1: Switch,
    switch_2: Switch,
}

impl BadRoom {
    pub fn new(tr: &Transport) -> BadRoom {
        BadRoom {
            chandelier: SerialSwitch::new(tr, "bedroom_lamp", 0x01),
            beam: WebBeam::new(tr, "bedroom_beam"),
            switch_1: Switch::new("bedroom_1", BadRoom::on_switch_1),
            switch_2: Switch::new("bedroom_1", BadRoom::on_switch_2),
        }
    }

    fn on_switch_1(this: &Home, action_type: ActionType) {}

    fn on_switch_2(this: &Home, action_type: ActionType) {}
}
