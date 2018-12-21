use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;

#[derive(Debug)]
pub struct WebController {
    devices: Arc<RwLock<HashMap<String, Arc<String>>>>
}

impl WebController {
    pub fn new() -> WebController {
        WebController {
            devices: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn reg_device(&self, ids: Vec<String>, host: String) {
        let mut map = self.devices.write().unwrap();
        let host = Arc::new(host);
        ids.into_iter().for_each(|id| {
            map.insert(id, host.clone());
        });
    }
}

impl Clone for WebController {
    fn clone(&self) -> Self {
        WebController { devices: self.devices.clone() }
    }
}