use anyhow::Error;
use dashmap::DashMap;
use rocksdb::DB;
use serde::de::DeserializeOwned;
use serde::export::fmt::Debug;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct Configuration {
    inner: Arc<DashMap<String, Box<dyn State>>>,
    store: Store,
}

impl Configuration {
    pub fn new(path: &str) -> Result<Configuration, Error> {
        Ok(Configuration {
            inner: Arc::new(Default::default()),
            store: Store::new(path)?,
        })
    }

    pub fn reg(&self, key: &str, cfg: impl State) -> Result<(), Error> {
        if let Some(val) = cfg.init(self.store.load::<Value>(key)?) {
            self.store.store(key, val);
        }
        self.inner.insert(key.to_owned(), Box::new(cfg));
        Ok(())
    }

    pub fn load_state(&self) -> Result<HashMap<String, Value>, Error> {
        self.inner
            .iter()
            .map(|r| r.key())
            .map(|key| self.store.load(key))
            .collect()
    }

    pub fn get_value(&self, key: &str) -> Result<Option<Value>, Error> {
        self.store.load(key)
    }

    pub fn set_value(&self, key: &str, value: Value) -> Result<Value, Error> {
        if let Some(mut val) = self.inner.get_mut(key) {
            let value = val.update(value)?;
            self.store.store(key, value.clone())?;
            Ok(value)
        } else {
            Err(Error::msg("Config not found"))
        }
    }
}

pub trait State: Debug + Send + Sync + 'static {
    fn init(&self, value: Option<Value>) -> Result<Option<Value>, Error>;
    fn update(&self, value: Value) -> Result<Value, Error>;
}

#[derive(Debug)]
pub struct Store {
    db: Arc<DB>,
}

impl Store {
    pub fn new(path: &str) -> Result<Store, Error> {
        Ok(Store {
            db: Arc::new(DB::open_default(path)?),
        })
    }

    pub fn store<V>(&self, key: &str, value: V) -> Result<(), Error>
    where
        V: Serialize,
    {
        Ok(self
            .db
            .put(key.as_bytes(), serde_json::to_string(value)?.as_bytes())?)
    }

    pub fn load<V>(&self, key: &str) -> Result<Option<V>, Error>
    where
        V: DeserializeOwned,
    {
        Ok(match self.db.get(key)? {
            Some(val) => serde_json::from_slice(&val)?,
            None => None,
        })
    }
}
