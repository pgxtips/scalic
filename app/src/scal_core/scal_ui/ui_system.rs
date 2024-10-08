use sdl2::event::Event;

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_system::ScalSystem, scal_ui::ui_component_tree::UIComponentTree, scal_window::ScalSDLWindow};

use super::{components::window_root::ComponentWindowRoot, ui_component_node::UIComponentNode, workspaces::{self, layout_one::default_workspace}};

pub struct UISystem {
    component_tree: UIComponentTree,
}

impl UISystem {
    pub fn new() -> Box<UISystem> {
        let mut component_tree = UIComponentTree::new();

        workspaces::layout_one::default_workspace(&mut component_tree);
        Box::new(UISystem {
            component_tree,
        })
    }
}

impl ScalSystem for UISystem {
    fn run_system(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {
        self.component_tree.render_tree_nodes(app_conf, win_handle);
    }
    fn update_system(&mut self, app_conf: &ApplicationConfig,  _win_handle: &mut ScalSDLWindow) {
        self.component_tree.update_tree_nodes(app_conf, _win_handle);
    }
    fn handle_input_system(&mut self, app_conf: &ApplicationConfig,  _win_handle: &mut ScalSDLWindow, events: &Vec<Event>) {
        self.component_tree.handle_input_tree_nodes(app_conf, _win_handle, events);
    }
}
