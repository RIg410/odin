use crate::home::Home;
use crate::devices::{LedState, Switch};
use serde_json::Value;
use crate::sensors::ActionType;
use anyhow::Result;
use std::collections::HashMap;
use crate::home::scripts::{Script, Runner};
use crate::home::scripts::switch::SWITCH_OFF_ALL;

pub fn scheme_scripts(scripts: &mut HashMap<String, Script>) {
    scripts.insert(
        "default_color_scheme".to_owned(),
        Script::new(default_color_scheme),
    );
    scripts.insert("color_scheme".to_owned(), Script::new(color_scheme));
}

fn all_beam(home: &Home, spot: Option<bool>, led: Option<LedState>) {
    home.living_room.beam.channel_1(spot, led);
    home.living_room.beam.channel_2(spot, led);

    home.corridor.beam.channel_1(spot, led);
    home.corridor.beam.channel_2(spot, led);

    home.kitchen.beam.channel_1(spot, led);
    home.kitchen.beam.channel_2(spot, led);
}

fn default_color_scheme(home: &Home, _value: Value) -> Result<()> {
    all_beam(home, Some(true), Some(LedState::default()));
    home.corridor.enable_ir();

    if home.living_room.beam.is_on() {
        home.living_room.beam.switch(true)?;
    }

    if home.corridor.beam.is_on() {
        home.corridor.beam.switch(true)?;
    }

    if home.kitchen.beam.is_on() {
        home.kitchen.beam.switch(true)?;
    }
    Ok(())
}

fn color_scheme(home: &Home, value: Value) -> Result<()> {
    let scheme: ColorScheme = serde_json::from_value(value)?;

    if let Some(enable_ir) = scheme.enable_ir {
        if enable_ir {
            home.corridor.enable_ir();
        } else {
            home.corridor.disable_ir();
            home.corridor.lamp.switch(false)?;
        }
    }

    all_beam(home, scheme.is_spot_on, scheme.led_mod);

    if scheme.switch_to {
        home.run_script(SWITCH_OFF_ALL, Value::Null)?;
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
