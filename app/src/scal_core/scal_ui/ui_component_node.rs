use sdl2::event::Event;

use crate::scal_core::{scal_application::configuration::ApplicationConfig, scal_ui::ui_component::UIComponent, scal_window::ScalSDLWindow};

pub struct UIComponentNode {
    pub component_name: String,
    pub component: Box<dyn UIComponent>,
    pub children: Vec<UIComponentNode>,
}

impl UIComponentNode {
    pub fn new(component_name: &str, component: Box<dyn UIComponent>) -> UIComponentNode {
        UIComponentNode {
            component_name: component_name.to_string(),
            component,
            children: Vec::new(),
        }
    }

    pub fn render_node(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {

        let draw_res = self.component.draw(app_conf, win_handle);

        if draw_res.is_err() {
            let e = draw_res.err().unwrap();
            log::error!("Error drawing component ({}): {}", self.component_name, e);
        }

        for child in self.children.iter() {
            child.render_node(app_conf, win_handle);
        }
    }

    // calls the handle_input on the current UIcomponent, then called 
    // handle_input on the children UIComponentNodes
    pub fn update_node(&mut self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {

        let update_res = self.component.update(app_conf, win_handle);

        if update_res.is_err() {
            let e = update_res.err().unwrap();
            log::error!("Error drawing component ({}): {}", self.component_name, e);
        }

        for child in self.children.iter_mut() {
            child.update_node(app_conf, win_handle);
        }
    }

    // calls the handle_input on the current UIcomponent, then called 
    // handle_input on the children UIComponentNodes
    pub fn handle_input_node(&mut self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow, events: &Vec<Event>) {

        let input_res = self.component.handle_input(app_conf, win_handle, events);

        if input_res.is_err() {
            let e = input_res.err().unwrap();
            log::error!("Error drawing component ({}): {}", self.component_name, e);
        }

        for child in self.children.iter_mut() {
            child.handle_input_node(app_conf, win_handle, events);
        }
    }

}
