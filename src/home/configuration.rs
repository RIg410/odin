use dashmap::DashMap;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct Configuration {
    inner: Arc<DashMap<String, Value>>
}