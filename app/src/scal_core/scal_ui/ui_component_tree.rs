
use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_ui::ui_component_node::UIComponentNode, scal_window::ScalSDLWindow};

pub struct UIComponentTree {
    pub root: Option<UIComponentNode>,
}

impl UIComponentTree {
    pub fn new() -> UIComponentTree {
        UIComponentTree {
            root: None,
        }
    }

    pub fn render_tree(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
        if self.root.is_some() {
            let root_node = self.root.as_ref().unwrap();
            root_node.render(app_conf, win_handle);
        }
    }
}
