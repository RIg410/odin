use std::collections::HashMap;
use controller::{Device, Switch};
use std::sync::Arc;

pub struct SwitchConfiguration {
    switch_map: HashMap<String, Arc<Switch>>
}

impl SwitchConfiguration {
    pub fn new(switch_vec: Vec<Arc<Switch>>) -> SwitchConfiguration {
        SwitchConfiguration {
            switch_map : switch_vec.iter()
            .map( | switch| {(switch.id.to_owned(), switch.clone())})
            .collect()
        }
    }

    pub fn get_switch(&self, switch_id : &str) -> Option<&Switch> {
       match self.switch_map.get(switch_id) {
           Some(sw) => Some(sw.as_ref()),
           None => None
       }
    }
}