use home::Home;
use io::IOBuilder;
use std::fmt::{Debug, Error, Formatter};
use std::string::ToString;
use std::sync::{Arc, RwLock};
use timer::time_ms;

pub type Action = dyn Fn(&Home, bool) -> Result<(), String> + Sync + Send + 'static;

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
pub struct Switch {
    pub id: Arc<String>,
    action: Arc<Action>,
    state: Arc<RwLock<SwitchState>>,
}

impl Switch {
    pub fn new<A>(io: &mut IOBuilder, id: &str, act: A) -> Switch
    where
        A: Fn(&Home, bool) -> Result<(), String> + Sync + Send + 'static,
    {
        let switch = Switch {
            id: Arc::new(id.to_string()),
            action: Arc::new(act),
            state: Arc::new(RwLock::new(SwitchState {
                is_on: false,
                last_update: 0,
            })),
        };

        io.add_sensor(switch.clone());
        switch
    }

    pub fn act(&self, home: &Home, action_type: ActionType) -> Result<(), String> {
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

impl Debug for Switch {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Switch {{ {} }}", self.id)
    }
}
