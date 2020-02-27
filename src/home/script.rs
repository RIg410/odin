use devices::LedState;
use devices::Switch as SwitchTrait;
use home::Home;
use sensors::ActionType;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Error, Formatter};

pub trait Runner {
    fn run_script(&self, name: &str, value: Value) -> Result<(), String>;
}

pub struct Script {
    inner: Box<dyn Fn(&Home, Value) -> Result<(), String> + Send + Sync + 'static>,
}

impl Script {
    fn new<A>(act: A) -> Script
    where
        A: Fn(&Home, Value) -> Result<(), String> + Send + Sync + 'static,
    {
        Script {
            inner: Box::new(act),
        }
    }

    pub fn run(&self, home: &Home, value: Value) -> Result<(), String> {
        (self.inner)(home, value)
    }
}

impl Debug for Script {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "action")
    }
}

pub fn scripts_map() -> HashMap<String, Script> {
    let mut map = HashMap::new();
    map.insert(
        "default_color_scheme".to_owned(),
        Script::new(default_color_scheme),
    );
    map.insert("color_scheme".to_owned(), Script::new(color_scheme));
    map
}

pub fn switch_off_all_switch(home: &Home) -> Result<(), String> {
    let corridor = &home.corridor;
    corridor.exit_1.act(home, ActionType::Off)?;

    let bad_room = &home.bad_room;
    bad_room.switch_1.act(home, ActionType::Off)?;
    bad_room.switch_2.act(home, ActionType::Off)?;

    let bathroom = &home.bathroom;
    bathroom.switch.act(home, ActionType::Off)?;

    let toilet = &home.toilet;
    toilet.switch.act(home, ActionType::Off)?;

    let kitchen = &home.kitchen;
    kitchen.switch_1.act(home, ActionType::Off)?;
    kitchen.switch_2.act(home, ActionType::Off)?;

    let balcony = &home.balcony;
    balcony.switch_1.act(home, ActionType::Off)?;
    balcony.switch_2.act(home, ActionType::Off)?;

    let living_room = &home.living_room;
    living_room.switch_1.act(home, ActionType::Off)?;
    living_room.switch_2.act(home, ActionType::Off)?;

    living_room.cupboard_lamp.switch(false);
    Ok(())
}

fn all_beam(home: &Home, spot: Option<bool>, led: Option<LedState>) {
    home.bad_room.beam.channel_1(spot, led);
    home.bad_room.beam.channel_2(spot, led);

    home.living_room.beam.channel_1(spot, led);
    home.living_room.beam.channel_2(spot, led);

    home.corridor.beam.channel_1(spot, led);
    home.corridor.beam.channel_2(spot, led);

    home.kitchen.beam.channel_1(spot, led);
    home.kitchen.beam.channel_2(spot, led);
}

fn default_color_scheme(home: &Home, _value: Value) -> Result<(), String> {
    all_beam(home, Some(true), Some(LedState::default()));
    home.corridor.enable_ir();

    if home.bad_room.beam.is_on() {
        home.bad_room.beam.switch(true);
    }

    if home.living_room.beam.is_on() {
        home.living_room.beam.switch(true);
    }

    if home.corridor.beam.is_on() {
        home.corridor.beam.switch(true);
    }

    if home.kitchen.beam.is_on() {
        home.kitchen.beam.switch(true);
    }
    Ok(())
}

fn color_scheme(home: &Home, value: Value) -> Result<(), String> {
    let scheme: ColorScheme = serde_json::from_value(value).map_err(|err| err.to_string())?;

    if let Some(enable_ir) = scheme.enable_ir {
        if enable_ir {
            home.corridor.enable_ir();
        } else {
            home.corridor.disable_ir();
            home.corridor.lamp.switch(false);
        }
    }

    all_beam(home, scheme.is_spot_on, scheme.led_mod);

    if scheme.switch_to {
        switch_off_all_switch(home)?;
        home.bad_room.switch_1.act(&home, ActionType::On)?;
        home.living_room.switch_1.act(&home, ActionType::On)?;
        home.corridor.exit_1.act(&home, ActionType::On)?;
        home.kitchen.switch_2.act(&home, ActionType::On)?;
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ColorScheme {
    led_mod: Option<LedState>,
    is_spot_on: Option<bool>,
    enable_ir: Option<bool>,
    switch_to: bool,
}
