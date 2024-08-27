use std::mem;

use crate::scal_core::scal_application::application::Application;
use crate::scal_core::scal_application::configuration::ApplicationConfig;
use crate::scal_core::scal_event::event_manager::EventManager;
use crate::scal_core::scal_system::ScalSystem;
use crate::scal_core::scal_window::ScalSDLWindow;

pub struct ConfigNotSet;
pub struct ConfigSet;
pub struct EventManagerNotSet;
pub struct EventManagerSet;
pub struct SystemsNotSet;
pub struct SystemsSet;

pub struct ApplicationBuilder<ConfigState, EventManagerState, SystemsState>{
    win_handle: Option<ScalSDLWindow>,
    event_manager: Option<EventManager>,
    systems: Option<Vec<Box<dyn ScalSystem>>>,
    config: Option<ApplicationConfig>,
    _config_state: std::marker::PhantomData<ConfigState>,
    _systems_state: std::marker::PhantomData<SystemsState>,
    _event_manager_state: std::marker::PhantomData<EventManagerState>,
}

impl ApplicationBuilder <ConfigNotSet, EventManagerNotSet, SystemsNotSet> {
    pub fn new() -> ApplicationBuilder<ConfigNotSet, EventManagerNotSet, SystemsNotSet> {
        ApplicationBuilder { 
            win_handle: None,
            event_manager: None,
            systems: None, 
            config: None,
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
            _event_manager_state: std::marker::PhantomData,
        }
    }

    pub fn set_config(self, config: ApplicationConfig) -> ApplicationBuilder<ConfigSet, EventManagerNotSet, SystemsNotSet> {
        ApplicationBuilder {
            win_handle: self.win_handle,
            event_manager: None,
            systems: None,
            config: Some(config),
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
            _event_manager_state: std::marker::PhantomData,
        }
    }
}

impl ApplicationBuilder <ConfigSet, EventManagerNotSet, SystemsNotSet> {
    pub fn set_event_manager(self, em: EventManager) -> ApplicationBuilder<ConfigSet, EventManagerSet, SystemsNotSet> {
        ApplicationBuilder {
            win_handle: self.win_handle,
            event_manager: Some(em),
            systems: None,
            config: self.config,
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
            _event_manager_state: std::marker::PhantomData,
        }
    }
}


impl ApplicationBuilder <ConfigSet, EventManagerSet, SystemsNotSet> {
    pub fn add_system(self, system: Box<dyn ScalSystem>) -> ApplicationBuilder<ConfigSet, EventManagerSet, SystemsSet> {
        ApplicationBuilder {
            win_handle: self.win_handle,
            event_manager: self.event_manager,
            systems: Some(vec![system]),
            config: self.config,
            _config_state: std::marker::PhantomData,
            _systems_state: std::marker::PhantomData,
            _event_manager_state: std::marker::PhantomData,
        }
    }
}

impl ApplicationBuilder <ConfigSet, EventManagerSet, SystemsSet> {
    pub fn add_system(mut self, system: Box<dyn ScalSystem>) -> Self {
        self.systems.as_mut().unwrap().push(system);
        self
    }

    pub fn build(self) -> anyhow::Result<Application> {
        let config = self.config.as_ref().unwrap();
        let window = ScalSDLWindow::new(config)?;

        Ok(Application {
            win_handle: window,
            event_manager: self.event_manager.unwrap(),
            systems: mem::take(&mut self.systems.unwrap()),
            config: self.config.unwrap(),
        })
    }
}
