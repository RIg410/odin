use std::collections::HashMap;

pub struct SwitchConfiguration {
    switch_map: HashMap<String, Vec<String>>
}

impl SwitchConfiguration {
    pub fn new() -> SwitchConfiguration {
        let mut map = HashMap::new();
        map.insert("switch_1".to_owned(), vec!("spot_1".to_owned(), "spot_2".to_owned()));


        SwitchConfiguration {
            switch_map: map
        }
    }

    pub fn get_lights_ids(&self, switch_id : &str) -> Option<&Vec<String>> {
        self.switch_map.get(switch_id)
    }
}