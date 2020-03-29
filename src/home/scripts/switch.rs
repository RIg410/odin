use crate::devices::Switch;
use crate::home::scripts::Script;
use crate::home::Home;
use crate::log_error;
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;

pub const SWITCH_OFF_ALL: &str = "switch_off_all";

pub fn switch_scripts(scripts: &mut HashMap<String, Script>) {
    scripts.insert(SWITCH_OFF_ALL.to_owned(), Script::new(switch_off_all));
}

pub fn switch_off_all(home: &Home, _value: Value) -> Result<()> {
    let corridor = &home.corridor;
    log_error!(corridor.beam.switch(false));

    let bad_room = &home.bad_room;
    log_error!(bad_room.chandelier.switch(false));

    let bathroom = &home.bathroom;
    log_error!(bathroom.lamp.switch(false));
    log_error!(bathroom.fun.switch(false));

    let toilet = &home.toilet;
    log_error!(toilet.fun.switch(false));
    log_error!(toilet.lamp.switch(false));

    let kitchen = &home.kitchen;
    log_error!(kitchen.beam.switch(false));
    log_error!(kitchen.kitchen_lamp.switch(false));

    let balcony = &home.balcony;
    log_error!(balcony.lamp.switch(false));

    let living_room = &home.living_room;
    log_error!(living_room.chandelier.switch(false));
    log_error!(living_room.beam.switch(false));
    log_error!(living_room.cupboard_lamp.switch(false));

    Ok(())
}
