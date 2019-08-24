use std::string::ToString;
use home::Home;
use std::fmt::{Debug, Formatter, Error};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use io::IOBuilder;

pub type Action = dyn Fn(&Home, bool) -> Result<(), String> + Sync + Send + 'static;

#[derive(Clone)]
pub struct Switch {
    pub id: Arc<String>,
    action: Arc<Action>,
    is_on: Arc<AtomicBool>,
}

impl Switch {
    pub fn new<A>(io: &mut IOBuilder, id: &str, act: A) -> Switch
        where A: Fn(&Home, bool) -> Result<(), String> + Sync + Send + 'static {
        let switch = Switch {
            id: Arc::new(id.to_string()),
            action: Arc::new(act),
            is_on: Arc::new(AtomicBool::new(false)),
        };

        io.add_sensor(switch.clone());
        switch
    }

    pub fn act(&self, home: &Home, action_type: ActionType) -> Result<(), String> {
        match action_type {
            ActionType::On => {
                self.is_on.store(true, Ordering::SeqCst);
                println!("On:{}", self.is_on.load(Ordering::SeqCst));
            }
            ActionType::Off => {
                self.is_on.store(false, Ordering::SeqCst);
                println!("Off:{}", self.is_on.load(Ordering::SeqCst));
            }
            ActionType::Toggle => {
                self.is_on.store(!self.is_on.load(Ordering::SeqCst), Ordering::SeqCst);
                println!("Toggle:{}", self.is_on.load(Ordering::SeqCst));
            }
        }
        (self.action)(home, self.is_on.load(Ordering::SeqCst))
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