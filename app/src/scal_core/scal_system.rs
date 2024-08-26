use crate::scal_core::scal_window::ScalSDLWindow;
use crate::scal_core::scal_application::configuration::ApplicationConfig;

pub trait ScalSystem {
    fn run(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow);
    fn update(&self, win_handle: &mut ScalSDLWindow);
    fn handle_input(&self, win_handle: &mut ScalSDLWindow);
}
