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

    pub fn render(&self, app_conf: &ApplicationConfig, win_handle: &mut ScalSDLWindow) {

        let draw_res = self.component.draw(app_conf, win_handle);

        if draw_res.is_err() {
            let e = draw_res.err().unwrap();
            log::error!("Error drawing component ({}): {}", self.component_name, e);
        }

        for child in self.children.iter() {
            child.render(app_conf, win_handle);
        }
    }
}
