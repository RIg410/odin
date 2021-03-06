use crate::devices::{SerialDimmer, SerialSwitch, Switch as SwitchTrait};
use crate::home::Home;
use crate::io::IOMut;
use crate::log_error;
use crate::runtime::{time_ms, RtTimer};
use crate::sensors::Switch;
use anyhow::Result;
use std::sync::RwLock;
use std::time::Duration;

#[derive(Debug)]
pub struct Toilet {
    pub lamp: SerialDimmer,
    pub fun: SerialSwitch,
    pub switch: Switch,
    pub timer: RwLock<RtTimer>,
}

impl Toilet {
    pub fn new(io: &mut IOMut) -> Toilet {
        let lamp = SerialDimmer::new(io, "toilet_lamp", 0x02, 25, 100);
        let fun = SerialSwitch::new(io, "toilet_fun", 0x03);
        log_error!(lamp.switch(false));
        log_error!(fun.switch(false));

        Toilet {
            lamp,
            fun,
            switch: Switch::toggle(io, "toilet", Toilet::on_switch),
            timer: RwLock::new(RtTimer::new(io.rt(), false)),
        }
    }

    fn on_switch(home: &Home) -> Result<()> {
        let toilet = &home.toilet;
        if !toilet.lamp.is_on() {
            toilet.fun.switch(true)?;
            toilet.lamp.switch(true)?;
            toilet.timer.write().unwrap().stop();
        } else {
            if toilet.lamp.is_on() && time_ms() - toilet.switch.last_update() > 30 * 1000 {
                toilet.fun.switch(true)?;
                let fun = toilet.fun.clone();
                toilet
                    .timer
                    .write()
                    .unwrap()
                    .after(Duration::from_secs(60 * 3), move || {
                        log_error!(fun.switch(false));
                    });
            } else {
                toilet.fun.switch(false)?;
            }

            toilet.lamp.switch(false)?
        }
        Ok(())
    }
}
