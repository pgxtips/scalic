use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_ui::ui_component::UIComponent, scal_window::ScalSDLWindow};

pub struct ComponentPrimarySideBar;

impl UIComponent for ComponentPrimarySideBar {
    fn draw(&self, app_conf: &ApplicationConfig, _win_handle: &mut ScalSDLWindow) -> anyhow::Result<()> {
        "Primary Side Bar".to_string();
        Ok(())
    }
    fn update(&self) {
        "Primary Side Bar".to_string();
    }
    fn handle_input(&self) {
        "TextBuffer".to_string();
    }
}
