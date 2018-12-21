use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use curl::easy::Easy;
use std::time::Duration;

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

    pub fn send(&self, id: &str, args: String) {
        let devices = self.devices.read().unwrap();
        if let Some(host) = devices.get(id) {
            let mut handle = Easy::new();
            let url = format!("http://{}/{}/{}", host, id, args);
            handle.url(&url).unwrap();
            handle.timeout(Duration::new(1, 0));
            let perf_res = handle.perform();
            println!("{} => {:?}", url, perf_res)
        }
    }
}

impl Clone for WebController {
    fn clone(&self) -> Self {
        WebController { devices: self.devices.clone() }
    }
}