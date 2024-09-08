use std::{cell::RefCell, rc::Rc};

use crate::rope::rope_node::{rope_node::RopeNode, rope_node_helper::print_tree };


#[allow(dead_code)]
fn create_example_rope() -> Rc<RefCell<RopeNode>> {

    let root = RopeNode::new(); 
    let b = RopeNode::new(); 
    let c = RopeNode::new(); 
    let d = RopeNode::new(); 
    let g = RopeNode::new(); 
    let h = RopeNode::new(); 

    let e = RopeNode::new_leaf("Hello_".to_string()); 
    let f = RopeNode::new_leaf("my_".to_string()); 
    let j = RopeNode::new_leaf("na".to_string()); 
    let k = RopeNode::new_leaf("me_i".to_string()); 
    let m = RopeNode::new_leaf("s".to_string()); 
    let n = RopeNode::new_leaf("_Simon".to_string()); 

    // set values, weights and make internal nodes
    //root.borrow_mut().set_weight(22);
    //b.borrow_mut().set_weight(9);
    //c.borrow_mut().set_weight(6);
    //d.borrow_mut().set_weight(6);
    //e.borrow_mut().set_weight(6);
    //f.borrow_mut().set_weight(3);
    //g.borrow_mut().set_weight(2);
    //h.borrow_mut().set_weight(1);
    //j.borrow_mut().set_weight(2);
    //k.borrow_mut().set_weight(4);
    //m.borrow_mut().set_weight(1);
    //n.borrow_mut().set_weight(6);

    // make connections

    RopeNode::set_left(Rc::clone(&root), Rc::clone(&b));

    RopeNode::set_left(Rc::clone(&b), Rc::clone(&c));
    RopeNode::set_right(Rc::clone(&b), Rc::clone(&d));

    RopeNode::set_left(Rc::clone(&c), Rc::clone(&e));
    RopeNode::set_right(Rc::clone(&c), Rc::clone(&f));

    RopeNode::set_left(Rc::clone(&d), Rc::clone(&g));
    RopeNode::set_right(Rc::clone(&d), Rc::clone(&h));

    RopeNode::set_left(Rc::clone(&g), Rc::clone(&j));
    RopeNode::set_right(Rc::clone(&g), Rc::clone(&k));

    RopeNode::set_left(Rc::clone(&h), Rc::clone(&m));
    RopeNode::set_right(Rc::clone(&h), Rc::clone(&n));

    let root_weight = RopeNode::get_weight(Rc::clone(&root));
    assert_eq!(22, root_weight);

    return root;
}

#[test]
fn rope_print_tree(){
    let example_rope = create_example_rope();
    print_tree(Some(example_rope.clone()));
    // RopeNode::rebalance(example_rope.clone());
    // print_tree(Some(example_rope.clone()));

}

