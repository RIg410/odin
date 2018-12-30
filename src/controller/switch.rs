use std::{
    sync::Arc,
    borrow::Cow,
    collections::HashMap,
};
use controller::{ActionType, DeviceBox};
use controller::Device;

pub type Action = Fn(ActionType) + Sync + Send + 'static;

pub struct Switch {
    id: Arc<String>,
    act: Arc<Action>,
}

impl Switch {
    pub fn empty<'a, ID>(id: ID) -> Switch
        where ID: Into<Cow<'a, str>>, {
        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(|_| {}) }
    }

    pub fn lambda<'a, ID, ACT>(id: ID, act: ACT) -> Switch
        where ID: Into<Cow<'a, str>>,
              ACT: Fn(ActionType) + Sync + Send + 'static {
        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(act) }
    }

    pub fn device<'a, ID>(id: ID, dev: DeviceBox) -> Switch
        where ID: Into<Cow<'a, str>> {
        Switch { id: Arc::new(id.into().to_string()), act: Arc::new(move |t| dev.switch(&t)) }
    }

    pub fn devices2<'a, ID>(id: ID, dev_1: DeviceBox, dev_2: DeviceBox) -> Switch
        where ID: Into<Cow<'a, str>> {
        Switch {
            id: Arc::new(id.into().to_string()),
            act: Arc::new(move |t| {
                dev_1.switch(&t);
                dev_2.switch(&t);
            }),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn on(&self) {
        (self.act)(ActionType::On)
    }

    pub fn off(&self) {
        (self.act)(ActionType::Off)
    }
}

impl Clone for Switch {
    fn clone(&self) -> Self {
        Switch {
            id: self.id.clone(),
            act: self.act.clone(),
        }
    }
}

pub struct SwitchHandler {
    switch_map: Arc<HashMap<String, Switch>>
}

impl SwitchHandler {
    pub fn new(switch_list: Vec<Switch>) -> SwitchHandler {
        let switch_map = switch_list.into_iter()
            .map(|switch| (switch.id.as_str().to_owned(), switch))
            .collect();

        SwitchHandler {
            switch_map: Arc::new(switch_map)
        }
    }

    pub fn switch(&self, name: &str, action_type: ActionType) {
        if let Some(act) = self.switch_map.get(name) {
            (act.act)(action_type)
        }
    }
}

impl Clone for SwitchHandler {
    fn clone(&self) -> Self {
        SwitchHandler {
            switch_map: self.switch_map.clone()
        }
    }
}