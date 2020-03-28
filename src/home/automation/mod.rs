use std::sync::Arc;
use crate::runtime::Background;
use crate::home::Home;
use crate::io::IO;
use crate::home::automation::web_beam_updater::WebBeamUpdater;

pub mod auto_shutdown;
pub mod web_beam_updater;

fn process() -> Vec<Box<dyn BackgroundBuilder>> {
    vec![
        Box::new(WebBeamUpdater()),
    ]
}


#[derive(Clone, Debug)]
pub struct BackgroundProcess {
    bg: Arc<Vec<Background>>
}

impl BackgroundProcess {
    pub fn new(home: &Home, io: &IO) -> BackgroundProcess {
        let bg = process().iter()
            .map(|b| b.build(home, io))
            .collect();

        BackgroundProcess {
            bg: Arc::new(bg)
        }
    }
}

pub trait BackgroundBuilder {
    fn build(&self, home: &Home, io: &IO) -> Background;
}