
use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_window::ScalSDLWindow};

/// A trait for UI components
/// render() - renders the component graphical representation
/// update() - updates the underlying data of the component
pub trait UIComponent {
    fn draw(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) -> anyhow::Result<()>;
    fn update(&self);
    fn handle_input(&self);
}
