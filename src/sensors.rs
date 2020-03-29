use crate::home::Home;
use crate::io::IOMut;
use crate::runtime::time_ms;
use anyhow::Result;
use std::fmt::{Debug, Error, Formatter};
use std::string::ToString;
use std::sync::{Arc, RwLock};

pub type StatefulAction = dyn Fn(&Home, bool) -> Result<()> + Sync + Send + 'static;
pub type StatelessAction = dyn Fn(&Home) -> Result<()> + Sync + Send + 'static;

#[derive(Clone, Debug)]
pub enum Switch {
    OnOff(StatefulSwitch),
    Toggle(Toggle),
}

impl Switch {
    pub fn new<A>(io: &mut IOMut, id: &str, act: A) -> Switch
    where
        A: Fn(&Home, bool) -> Result<()> + Sync + Send + 'static,
    {
        let switch = Switch::OnOff(StatefulSwitch {
            id: Arc::new(id.to_string()),
            action: Arc::new(act),
            state: Arc::new(RwLock::new(SwitchState {
                is_on: false,
                last_update: 0,
            })),
        });

        io.add_sensor(switch.clone());
        switch
    }

    pub fn toggle<A>(io: &mut IOMut, id: &str, act: A) -> Switch
    where
        A: Fn(&Home) -> Result<()> + Sync + Send + 'static,
    {
        let switch = Switch::Toggle(Toggle {
            id: Arc::new(id.to_string()),
            action: Arc::new(act),
            last_update: Arc::new(Default::default()),
        });

        io.add_sensor(switch.clone());
        switch
    }

    pub fn act(&self, home: &Home, action_type: ActionType) -> Result<()> {
        match self {
            Switch::OnOff(switch) => switch.act(home, action_type),
            Switch::Toggle(switch) => switch.act(home),
        }
    }

    pub fn last_update(&self) -> u128 {
        match self {
            Switch::OnOff(switch) => switch.last_update(),
            Switch::Toggle(switch) => *switch.last_update.read().unwrap(),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Switch::OnOff(switch) => switch.id.as_ref(),
            Switch::Toggle(switch) => switch.id.as_str(),
        }
    }
}

struct SwitchState {
    is_on: bool,
    last_update: u128,
}

impl SwitchState {
    pub fn cas(&mut self) {
        self.is_on = !self.is_on;
    }

    pub fn update_time(&mut self) {
        self.last_update = time_ms();
    }
}

#[derive(Clone)]
pub struct StatefulSwitch {
    pub id: Arc<String>,
    action: Arc<StatefulAction>,
    state: Arc<RwLock<SwitchState>>,
}

impl StatefulSwitch {
    pub fn act(&self, home: &Home, action_type: ActionType) -> Result<()> {
        let is_on = {
            let mut state = self.state.write().unwrap();
            match action_type {
                ActionType::On => state.is_on = true,
                ActionType::Off => state.is_on = false,
                ActionType::Toggle => state.cas(),
            }
            state.is_on
        };

        let res = (self.action)(home, is_on);
        self.state.write().unwrap().update_time();
        res
    }

    pub fn last_update(&self) -> u128 {
        self.state.read().unwrap().last_update
    }
}

pub enum ActionType {
    On,
    Off,
    Toggle,
}

impl Debug for StatefulSwitch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Switch {{ {} }}", self.id)
    }
}

#[derive(Clone)]
pub struct Toggle {
    pub id: Arc<String>,
    action: Arc<StatelessAction>,
    last_update: Arc<RwLock<u128>>,
}

impl Toggle {
    pub fn act(&self, home: &Home) -> Result<()> {
        let res = (self.action)(home);
        *self.last_update.write().unwrap() = time_ms();
        res
    }
}

impl Debug for Toggle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Switch {{ {} }}", self.id)
    }
}
