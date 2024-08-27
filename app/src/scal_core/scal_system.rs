use sdl2::event::Event;

use crate::scal_core::scal_window::ScalSDLWindow;
use crate::scal_core::scal_application::configuration::ApplicationConfig;

pub trait ScalSystem {
    fn run_system(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow);
    fn update_system(&mut self,app_conf: &ApplicationConfig,  win_handle: &mut ScalSDLWindow);
    fn handle_input_system(&mut self,app_conf: &ApplicationConfig,  win_handle: &mut ScalSDLWindow, events: &Vec<Event>);
}
