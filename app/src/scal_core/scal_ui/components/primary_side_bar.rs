use sdl2::event::Event;

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_ui::ui_component::UIComponent, scal_window::ScalSDLWindow};

pub struct ComponentPrimarySideBar;

impl UIComponent for ComponentPrimarySideBar {
    fn draw(&self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow) -> anyhow::Result<()> {
        "Primary Side Bar".to_string();
        Ok(())
    }
    fn update(&mut self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow) -> anyhow::Result<()> {
        "Primary Side Bar".to_string();
        Ok(())
    }
    fn handle_input(&mut self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow, events: &Vec<Event>) -> anyhow::Result<()>{
        "TextBuffer".to_string();
        Ok(())
    }
}
