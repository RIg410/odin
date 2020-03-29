use anyhow::Result;
use dashmap::DashMap;
use reqwest::blocking;
use std::fmt::Write;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct WebChannel {
    devices: Arc<DashMap<String, Arc<String>>>,
}

impl WebChannel {
    pub fn new() -> WebChannel {
        WebChannel {
            devices: Default::default(),
        }
    }

    pub fn reg_device(&self, ids: Vec<String>, host: String) {
        let host = Arc::new(host);
        ids.into_iter().for_each(|id| {
            self.devices.insert(id, host.clone());
        });
    }

    pub fn host(&self, id: &str) -> Option<Arc<String>> {
        if let Some(host) = self.devices.get(id) {
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
