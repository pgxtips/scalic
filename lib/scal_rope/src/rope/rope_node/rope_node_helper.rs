use std::{cell::RefCell, rc::Rc};

use super::rope_node::RopeNode;

#[allow(dead_code)]
pub fn print_tree(rope: Option<Rc<RefCell<RopeNode>>>){
    if rope.is_none() { return; }
    let unwrap_rope = Rc::clone(&rope.unwrap());

    let depth = RopeNode::max_depth(unwrap_rope.clone()); 

    for i in 1..depth+1 {

        let str_nodes = print_current_level(Some(unwrap_rope.clone()), i as usize, &mut Vec::new());

        let initial_spaces = 2_usize.pow( (depth - i) as u32);
        let initial_spaces_str = " ".repeat(initial_spaces);

        let between_spaces = 2_usize.pow( (depth - i + 1) as u32) - 1;
        let between_spaces_str = " ".repeat(between_spaces);

        print!("{}", initial_spaces_str);
        for node in &str_nodes { 
            print!("{}{}", node, between_spaces_str); 
        }

        print!("\n\n");

    }

}

fn print_current_level(node: Option<Rc<RefCell<RopeNode>>>, level: usize, print_value: &mut Vec<String>) -> Vec<String> {
    if node.is_none() { 
        // For missing nodes, we add a space
        print_value.push(" ".to_string()); 
        return print_value.to_vec();
    }

    let node = node.unwrap();

    if level == 1 {
        let weight = RopeNode::get_weight(Rc::clone(&node));

        let node_str = format!("{}", weight);
        print_value.push(node_str);
    }
    else if level > 1 {
        let left = RopeNode::get_left(Rc::clone(&node));
        let right = RopeNode::get_right(Rc::clone(&node));

        print_current_level(left, level - 1, print_value);
        print_current_level(right, level - 1, print_value);
    }

    print_value.to_vec()
}
