use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_ui::ui_component::UIComponent, scal_window::ScalSDLWindow};

pub struct ComponentTitleBar;

impl UIComponent for ComponentTitleBar {
    fn draw(&self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>{
        Ok(())
    }
    fn update(&self) {
        "Title Bar".to_string();
    }
    fn handle_input(&self) {
        "TextBuffer".to_string();
    }
}
