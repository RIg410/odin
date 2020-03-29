use crate::home::automation::web_beam_updater::WebBeamUpdater;
use crate::home::configuration::Configuration;
use crate::home::Home;
use crate::io::IO;
use crate::runtime::Background;
use anyhow::Error;
use std::sync::Arc;

pub mod auto_shutdown;
pub mod web_beam_updater;

fn process(
    _home: &Home,
    io: &IO,
    config: &Configuration,
) -> Result<Vec<Box<dyn BackgroundBuilder>>, Error> {
    Ok(vec![Box::new(WebBeamUpdater::new(io, config)?)])
}

#[derive(Clone, Debug)]
pub struct BackgroundProcess {
    bg: Arc<Vec<Background>>,
}

impl BackgroundProcess {
    pub fn new(home: &Home, io: &IO, config: &Configuration) -> Result<BackgroundProcess, Error> {
        let bg = process(home, io, config)?
            .into_iter()
            .map(|mut b| b.build())
            .collect();

        Ok(BackgroundProcess { bg: Arc::new(bg) })
    }
}

pub trait BackgroundBuilder {
    fn build(&mut self) -> Background;
}
