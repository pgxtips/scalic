use std::mem;

use crate::scal_core::scal_application::application::Application;
use crate::scal_core::scal_application::configuration::ApplicationConfig;
use crate::scal_core::scal_system::ScalSystem;
use crate::scal_core::scal_window::ScalSDLWindow;

pub struct ConfigNotSet;
pub struct ConfigSet;
pub struct SystemsNotSet;
pub struct SystemsSet;

pub struct ApplicationBuilder<ConfigState, SystemsState>{
    win_handle: Option<ScalSDLWindow>,
    systems: Option<Vec<Box<dyn ScalSystem>>>,
    config: Option<ApplicationConfig>,
    _config_state: std::marker::PhantomData<ConfigState>,
    _systems_state: std::marker::PhantomData<SystemsState>,
}

impl ApplicationBuilder <ConfigNotSet, SystemsNotSet> {
    pub fn new() -> ApplicationBuilder<ConfigNotSet, SystemsNotSet> {
        ApplicationBuilder { 
            win_handle: None,
            systems: None, 
            config: None,
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
        }
    }

    pub fn set_config(self, config: ApplicationConfig) -> ApplicationBuilder<ConfigSet, SystemsNotSet> {
        ApplicationBuilder {
            win_handle: self.win_handle,
            systems: None,
            config: Some(config),
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
        }
    }
}

impl ApplicationBuilder <ConfigSet, SystemsNotSet> {
    pub fn add_system(mut self, system: Box<dyn ScalSystem>) -> ApplicationBuilder<ConfigSet, SystemsSet> {
        ApplicationBuilder {
            win_handle: self.win_handle,
            systems: Some(vec![system]),
            config: self.config,
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
        }
    }
}

impl ApplicationBuilder <ConfigSet, SystemsSet> {
    pub fn add_system(mut self, system: Box<dyn ScalSystem>) -> Self {
        self.systems.as_mut().unwrap().push(system);
        self
    }

    pub fn build(mut self) -> anyhow::Result<Application> {
        let config = self.config.as_ref().unwrap();
        let window = ScalSDLWindow::new(config)?;

        Ok(Application {
            win_handle: window,
            systems: mem::take(&mut self.systems.unwrap()),
            config: self.config.unwrap(),
        })
    }
}
