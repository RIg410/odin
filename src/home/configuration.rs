use anyhow::Error;
use dashmap::DashMap;
use serde::export::fmt::Debug;
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Configuration {
    inner: Arc<DashMap<String, ConfigValue>>,
}

impl Configuration {
    pub fn add(&self, key: &str, cfg: ConfigValue) {
        self.inner.insert(key.to_owned(), cfg);
    }

    pub fn get_state(&self) -> HashMap<String, Value> {
        self.inner.iter()
            .map(|r| (r.key().to_owned(), r.value.clone()))
            .collect()
    }

    pub fn get_value(&self, key: &str) -> Option<Value> {
        self.inner.get(key).map(|v| v.value.clone())
    }

    pub fn set_value(&self, key: &str, value: Value) -> Result<Value, Error> {
        if let Some(mut val) = self.inner.get_mut(key) {
            val.update(value)
        } else {
            Err(Error::msg("Config not found"))
        }
    }
}

#[derive(Debug)]
pub struct ConfigValue {
    value: Value,
    on_update: Box<dyn OnUpdate>,
}

impl ConfigValue {
    pub fn new<V, U>(val: V, on_update: U) -> Result<ConfigValue, Error>
        where
            V: Serialize,
            U: OnUpdate + 'static,
    {
        Ok(ConfigValue {
            value: serde_json::to_value(val)?,
            on_update: Box::new(on_update),
        })
    }

    pub fn update(&mut self, new_value: Value) -> Result<Value, Error> {
        let value = self.on_update.on_update(new_value.clone())?;
        self.value = value.clone();
        Ok(value)
    }
}

pub trait OnUpdate: Debug + Send + Sync + 'static {
    fn on_update(&self, value: Value) -> Result<Value, Error>;
}
