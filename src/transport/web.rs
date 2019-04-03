use std::{
    sync::Arc,
    sync::RwLock,
    collections::HashMap,
    time::Duration
};
use actix_web::{actix, client};
use futures::future::Future;

#[derive(Debug)]
pub struct WebChannel {
    devices: Arc<RwLock<HashMap<String, Arc<String>>>>
}

impl WebChannel {
    pub fn new() -> WebChannel {
        WebChannel {
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

    pub fn send(&self, id: &str, arg: &str, arg2: &str) {
        if let Some(host) = self.host(id) {
            let url = format!("http://{}/{}?arg_1={}&arg_2={}", &host, id, arg, arg2);
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

impl Clone for WebChannel {
    fn clone(&self) -> Self {
        WebChannel { devices: self.devices.clone() }
    }
}