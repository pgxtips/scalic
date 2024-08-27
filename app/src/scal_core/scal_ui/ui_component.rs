
use sdl2::event::Event;

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_window::ScalSDLWindow};

/// A trait for UI components
/// render() - renders the component graphical representation
/// update() - updates the underlying data of the component
pub trait UIComponent {
    fn draw(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>;
    fn update(&mut self,  app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>;
    fn handle_input(&mut self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow, events: &Vec<Event>) -> anyhow::Result<()>;
}
