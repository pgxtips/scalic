
use sdl2::event::Event;

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

    pub fn render_tree_nodes(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
        if self.root.is_some() {
            let root_node = self.root.as_ref().unwrap();
            root_node.render_node(app_conf, win_handle);
        }
    }

    pub fn update_tree_nodes(&mut self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
        if self.root.is_some() {
            let root_node = self.root.as_mut().unwrap();
            root_node.update_node(app_conf, win_handle);
        }
    }

    pub fn handle_input_tree_nodes(&mut self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow, events: &Vec<Event>) {
        if self.root.is_some() {
            let root_node = self.root.as_mut().unwrap();
            root_node.handle_input_node(app_conf, win_handle, events);
        }
    }
}
