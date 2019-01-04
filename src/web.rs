use std::{
    sync::Arc,
    sync::RwLock,
    collections::HashMap,
    time::Duration
};
use actix_web::{actix, client};
use futures::future::Future;

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

    pub fn host(&self, id: &str) -> Option<Arc<String>> {
        let devices = self.devices.read().unwrap();
        if let Some(host) = devices.get(id) {
            Some(host.clone())
        } else {
            None
        }
    }

    pub fn send(&self, id: &str, args: String) {
        if let Some(host) = self.host(id) {
            let url = format!("http://{}/{}?args={}", &host, id, args);
            println!("req => {:?}", url);
            actix::spawn(
                client::get(url)
                    .timeout(Duration::new(1, 0))
                    .finish().unwrap()
                    .send()
                    .map_err(|_| ())
                    .and_then(|response| {
                        println!("resp => {:?}", response);
                        Ok(())
                    })
            );
        }
    }
}

impl Clone for WebController {
    fn clone(&self) -> Self {
        WebController { devices: self.devices.clone() }
    }
}