use anyhow::Result;
use reqwest::blocking;
use std::fmt::Write;
use std::time::Duration;
use std::{collections::HashMap, sync::Arc, sync::RwLock};

#[derive(Debug)]
pub struct WebChannel {
    devices: Arc<RwLock<HashMap<String, Arc<String>>>>,
}

impl WebChannel {
    pub fn new() -> WebChannel {
        WebChannel {
            devices: Arc::new(RwLock::new(HashMap::new())),
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

    pub fn send(&self, id: &str, args: Vec<String>) -> Result<()> {
        if let Some(host) = self.host(id) {
            let mut url = String::new();
            write!(url, "http://{}/{}?", &host, id)?;
            for (i, arg) in args.iter().enumerate() {
                write!(url, "arg_{}={}&", i, arg)?;
            }
            url.pop();
            let resp = blocking::Client::builder()
                .timeout(Duration::from_secs(1))
                .build()?
                .get(&url)
                .send();
            debug!("resp => {:?}", resp);
        } else {
            debug!("Unknown device:{}", id)
        }
        Ok(())
    }
}

impl Clone for WebChannel {
    fn clone(&self) -> Self {
        WebChannel {
            devices: self.devices.clone(),
        }
    }
}
