
/*
#[test]
fn creating_a_new_rope() {
    use crate::rope::rope::Rope;

    let rope = Rope::new();
    assert_eq!(rope.root, Some(Box::new(RopeNode::new())));
}
*/

use std::{cell::RefCell, rc::Rc};
use crate::rope::rope_node_new::RopeNode;

fn create_example_rope() -> Rc<RefCell<RopeNode>> {

    let root = RopeNode::new(); 
    let b = RopeNode::new(); 
    let c = RopeNode::new(); 
    let d = RopeNode::new(); 
    let e = RopeNode::new_leaf("Hello_".to_string()); 
    let f = RopeNode::new_leaf("my_".to_string()); 
    let g = RopeNode::new(); 
    let h = RopeNode::new(); 
    let j = RopeNode::new_leaf("na".to_string()); 
    let k = RopeNode::new_leaf("me_i".to_string()); 
    let m = RopeNode::new_leaf("s".to_string()); 
    let n = RopeNode::new_leaf("_Simon".to_string()); 

    // set values, weights and make internal nodes
    root.borrow_mut().set_weight(22);
    b.borrow_mut().set_weight(9);
    c.borrow_mut().set_weight(6);
    d.borrow_mut().set_weight(6);
    e.borrow_mut().set_weight(6);
    f.borrow_mut().set_weight(3);
    g.borrow_mut().set_weight(2);
    h.borrow_mut().set_weight(1);
    j.borrow_mut().set_weight(2);
    k.borrow_mut().set_weight(4);
    m.borrow_mut().set_weight(1);
    n.borrow_mut().set_weight(6);

    // make connections

    root.borrow_mut().set_left(b.clone());
    root.borrow_mut().set_right_none();

    b.borrow_mut().set_left(c.clone());
    b.borrow_mut().set_right(d.clone());

    c.borrow_mut().set_left(e);
    c.borrow_mut().set_right(f);

    d.borrow_mut().set_left(g.clone());
    d.borrow_mut().set_right(h.clone());

    g.borrow_mut().set_left(j);
    g.borrow_mut().set_right(k);

    h.borrow_mut().set_left(m);
    h.borrow_mut().set_right(n);

    return root;
}

#[test]
fn rope_traversal(){

    let example_rope = create_example_rope();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let actual = example_rope
        .borrow()
        .iter()
        .map(|node| node.borrow().get_value().unwrap())
        .collect::<Vec<String>>();

    println!("\n{:?}\n", actual);
    assert_eq!(expected, actual);
}

#[test]
fn rope_get_length(){

    let rope = create_example_rope();
    let length = rope.borrow().get_length();

    assert_eq!(22, length);
}

#[test]
fn rope_max_depth(){
    let example_rope = create_example_rope();
    let depth = example_rope.borrow().max_depth();

    assert_eq!(5, depth);
}

#[test]
fn rope_concat_ideal(){

    let s1_1 = RopeNode::new_leaf("Hello_".to_string());
    let s1_2 = RopeNode::new_leaf("my_".to_string());

    let r1 = RopeNode::new();
    r1.borrow_mut().set_weight(6);
    r1.borrow_mut().set_left(s1_1);
    r1.borrow_mut().set_right(s1_2);


    let s2_1 = RopeNode::new_leaf("na".to_string()); 
    let s2_2 = RopeNode::new_leaf("me_i".to_string());

    let r2 = RopeNode::new();
    r2.borrow_mut().set_weight(2);
    r2.borrow_mut().set_left(s2_1);
    r2.borrow_mut().set_right(s2_2);

    let final_rope = r1.borrow().concat(r2).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i" ];
    let result = final_rope
        .borrow()
        .iter()
        .map(|node| node.borrow().get_value().unwrap())
        .collect::<Vec<String>>();

    println!("\nconcat result: {:?}\n", result);

    assert_eq!(expected, result);
    assert_eq!(9, final_rope.borrow().get_weight());
}

#[test]
fn rope_is_balanced_ideal(){

    let r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();

    r1.borrow_mut().set_left(n1);
    r1.borrow_mut().set_right(n2);

    assert_eq!(true, r1.borrow().is_balanced());
}

#[test]
fn rope_is_balanced_not_ideal(){

    let r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();

    r1.borrow_mut().set_left(Rc::clone(&n1));
    n1.borrow_mut().set_left(n2);

    assert_eq!(false, r1.borrow().is_balanced());
}

#[test]
fn rope_is_balanced_not_ideal_2(){

    let r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();
    let n3 = RopeNode::new();
    let n4 = RopeNode::new();
    let n5 = RopeNode::new_leaf("leaf".to_string());

    n4.borrow_mut().set_left(Rc::clone(&n5));
    n3.borrow_mut().set_left(Rc::clone(&n4));
    n1.borrow_mut().set_right(Rc::clone(&n3));

    r1.borrow_mut().set_left(Rc::clone(&n1));
    r1.borrow_mut().set_right(Rc::clone(&n2));

    assert_eq!(false, r1.borrow().is_balanced());
}

