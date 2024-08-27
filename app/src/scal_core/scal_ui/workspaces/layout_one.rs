use crate::scal_core::{self, scal_ui::{components::{text_buffer::ComponentTextBuffer, window_root::ComponentWindowRoot}, ui_component_node::UIComponentNode, ui_component_tree::UIComponentTree}};


pub fn default_workspace(component_tree: &mut UIComponentTree) {

    let mut window_root = UIComponentNode::new("window_root", Box::new(ComponentWindowRoot{
        x: 0,
        y: 0,
        width: 800,
        height: 600,
        colour: (0, 0, 0, 0)
    }));

    let buffer = scal_core::scal_buffer::buffer::Buffer::new_test_one(); 
    //let buffer = scal_core::scal_buffer::buffer::Buffer::read("test_file.txt").unwrap(); 

    let text_buffer = UIComponentNode::new("text_buffer_1", Box::new(ComponentTextBuffer{
        buffer: Some(buffer),
    }));

    window_root.children.push(text_buffer);

    component_tree.root = Some(window_root);
}
