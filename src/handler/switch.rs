use controller::CommonSwitch;
use std::sync::Arc;
use std::collections::HashMap;
use controller::Switch;

pub struct SwitchHolder {
    switch_map: HashMap<String, Box<Switch>>
}

impl SwitchHolder {
    pub fn new(switch_vec: Vec<Box<Switch>>) -> SwitchHolder {
        SwitchHolder {
            switch_map: switch_vec.iter()
                .map(|switch| { (switch.id().to_owned(), switch.clone()) })
                .collect()
        }
    }

    pub fn get_switch(&self, switch_id: &str) -> Option<&Switch> {
        match self.switch_map.get(switch_id) {
            Some(sw) => Some(&sw),
            None => None
        }
    }
}

pub struct SwitchHandler {
    switch_holder: Arc<SwitchHolder>,
}

impl SwitchHandler {
    pub fn new(config: Arc<SwitchHolder>) -> SwitchHandler {
        SwitchHandler { switch_holder: config }
    }

    #[inline]
    fn get_switch(&self, switch_id: &str) -> Result<&Switch, Option<String>> {
        let switch = self.switch_holder.get_switch(switch_id);
        if let Some(switch) = switch {
            Ok(switch)
        } else {
            Err(Some(format!("There are not lights for switch with id :{}", switch_id)))
        }
    }

    pub fn handle(&self, switch_id: &str, state: &str) {
        if let Ok(switch) = self.get_switch(switch_id) {
            match state {
                "ON" => {
                    switch.switch_on();
                }
                "OFF" => {
                    switch.switch_off();
                }
                a @ _ => {
                    println!("Unsupported action: [{}]", a);
                }
            }
        } else {
            println!("Switch not found:{}", switch_id);
        }
    }
}