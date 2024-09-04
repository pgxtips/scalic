use std::{cell::RefCell, rc::Rc};
use crate::rope::rope_node::RopeNode;

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
fn rope_traversal(){

    let example_rope = create_example_rope();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(example_rope);

    //println!("\n{:?}\n", actual);
    assert_eq!(expected, actual);
}

#[test]
fn rope_get_length(){

    let rope = create_example_rope();
    let length = RopeNode::get_length(rope.clone());

    assert_eq!(22, length);
}

#[test]
fn rope_max_depth(){
    let example_rope = create_example_rope();
    let depth = RopeNode::max_depth(example_rope);

    assert_eq!(5, depth);
}

#[test]
fn rope_concat_ideal(){

    let s1_1 = RopeNode::new_leaf("Hello_".to_string());
    let s1_2 = RopeNode::new_leaf("my_".to_string());

    let r1 = RopeNode::new();
    RopeNode::set_left(Rc::clone(&r1), Rc::clone(&s1_1));
    RopeNode::set_right(Rc::clone(&r1), Rc::clone(&s1_2));


    let s2_1 = RopeNode::new_leaf("na".to_string()); 
    let s2_2 = RopeNode::new_leaf("me_i".to_string());

    let r2 = RopeNode::new();
    RopeNode::set_left(Rc::clone(&r2), Rc::clone(&s2_1));
    RopeNode::set_right(Rc::clone(&r2), Rc::clone(&s2_2));

    println!("rope_1 weight: {}", RopeNode::get_weight(Rc::clone(&r1)));
    println!("rope_2 weight: {}", RopeNode::get_weight(Rc::clone(&r2)));

    let final_rope = RopeNode::concat(Rc::clone(&r1), Some(Rc::clone(&r2))).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i" ];
    let result = RopeNode::get_leaves(Rc::clone(&final_rope));

    println!("\nconcat result: {:?}\n", result);

    assert_eq!(expected, result);
    assert_eq!(9, RopeNode::get_weight(Rc::clone(&final_rope)));
}

#[test]
fn rope_is_balanced_ideal(){

    let r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();

    RopeNode::set_left(Rc::clone(&r1), Rc::clone(&n1));
    RopeNode::set_right(Rc::clone(&r1), Rc::clone(&n2));

    assert_eq!(true, RopeNode::is_balanced(Rc::clone(&r1)));
}

#[test]
fn rope_is_balanced_not_ideal(){

    let r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();

    RopeNode::set_left(Rc::clone(&r1), Rc::clone(&n1));
    RopeNode::set_left(Rc::clone(&n1), Rc::clone(&n2));

    assert_eq!(false, RopeNode::is_balanced(Rc::clone(&r1)));
}

#[test]
fn rope_is_balanced_not_ideal_2(){

    let r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();
    let n3 = RopeNode::new();
    let n4 = RopeNode::new();
    let n5 = RopeNode::new_leaf("leaf".to_string());

    RopeNode::set_left(Rc::clone(&n4), Rc::clone(&n5));
    RopeNode::set_left(Rc::clone(&n3), Rc::clone(&n4));
    RopeNode::set_right(Rc::clone(&n1), Rc::clone(&n3));

    RopeNode::set_left(Rc::clone(&r1), Rc::clone(&n1));
    RopeNode::set_right(Rc::clone(&r1), Rc::clone(&n2));

    assert_eq!(false, RopeNode::is_balanced(Rc::clone(&r1)));
}

#[test]
fn rope_rebalance(){
    let rope = create_example_rope();
    assert_eq!(false, RopeNode::is_balanced(Rc::clone(&rope)));

    RopeNode::rebalance(Rc::clone(&rope));
    assert_eq!(true, RopeNode::is_balanced(Rc::clone(&rope)));

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(Rc::clone(&rope));

    assert_eq!(expected, actual);
}

#[test]
fn rope_index_of_ideal(){
    let rope = create_example_rope();
    let idx = 7;

    let result = RopeNode::index_of(Rc::clone(&rope), idx).unwrap();
    assert_eq!('y', result);
}

#[test]
fn rope_index_of_upper_bound(){
    let rope = create_example_rope();
    let values = RopeNode::get_leaves(Rc::clone(&rope)).join("");
    let idx = values.len() - 1;

    let result = RopeNode::index_of(Rc::clone(&rope), idx).unwrap();
    assert_eq!('n', result);
}

#[test]
fn rope_index_of_lower_bound(){
    let rope = create_example_rope();
    let idx = 0;

    let result = RopeNode::index_of(Rc::clone(&rope), idx).unwrap();
    assert_eq!('H', result);
}

#[test]
fn rope_index_of_out_of_bounds_large(){
    let rope = create_example_rope();
    let idx = 22;
    let result = RopeNode::index_of(Rc::clone(&rope),idx);

    assert_eq!(true, result.is_err());
}


/*
let left_vals = left
    .borrow()
    .iter()
    .filter(|node| node.borrow().get_value().is_some())
    .map(|node| node.borrow().get_value().unwrap())
    .collect::<Vec<String>>();
*/


#[test]
fn rope_split_lower_bound(){
    let rope = create_example_rope();

    let (left, right) = RopeNode::split(rope, 0).unwrap();

    let left_vals = RopeNode::get_leaves(left.clone()); 
    let right_vals = RopeNode::get_leaves(right.clone());

    println!("\nleft: {:?}", left_vals);
    println!("right: {:?}\n", right_vals);


    let left_split_expected = vec![""];
    let right_split_expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];

    assert_eq!(left_split_expected, left_vals);
    assert_eq!(right_split_expected, right_vals);
}

#[test]
fn rope_split_first_letter(){
    let rope = create_example_rope();

    let (left, right) = RopeNode::split(rope, 11).unwrap();

    let left_vals = RopeNode::get_leaves(left.clone()); 
    let right_vals = RopeNode::get_leaves(right.clone());

    println!("\nleft: {:?}", left_vals);
    println!("right: {:?}\n", right_vals);


    let left_split_expected = vec![ "Hello_", "my_", "na"];
    let right_split_expected = vec![ "me_i", "s", "_Simon"];

    assert_eq!(left_split_expected, left_vals);
    assert_eq!(right_split_expected, right_vals);
}

#[test]
fn rope_split_mid_letter(){
    let rope = create_example_rope();

    let (left, right) = RopeNode::split(rope.clone(), 13).unwrap();

    let left_vals = RopeNode::get_leaves(left.clone()); 
    let right_vals = RopeNode::get_leaves(right.clone());

    println!("\nleft_split: {:?}", left_vals);
    println!("right_split: {:?}\n", right_vals);

    let left_split_expected = vec![ "Hello_", "my_", "na", "me"];
    let right_split_expected = vec![ "_i", "s", "_Simon"];

    assert_eq!(left_split_expected, left_vals);
    assert_eq!(right_split_expected, right_vals);
}


#[test]
fn rope_split_last_letter(){
    let rope = create_example_rope();

    let (left, right) = RopeNode::split(rope.clone(), 15).unwrap();

    let left_vals = RopeNode::get_leaves(left.clone()); 
    let right_vals = RopeNode::get_leaves(right.clone());

    println!("\nleft: {:?}\n", left_vals);
    println!("\nright: {:?}\n", right_vals);

    let left_split_expected = vec![ "Hello_", "my_", "na", "me_i"];
    let right_split_expected = vec![ "s", "_Simon"];

    assert_eq!(left_split_expected, left_vals);
    assert_eq!(right_split_expected, right_vals);
}

#[test]
fn rope_split_upper_bound(){
    let rope = create_example_rope();

    let (left, right) = RopeNode::split(rope, 22).unwrap();

    let left_vals = RopeNode::get_leaves(left.clone()); 
    let right_vals = RopeNode::get_leaves(right.clone());

    println!("\nleft: {:?}", left_vals);
    println!("right: {:?}\n", right_vals);


    let left_split_expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon" ];
    let right_split_expected = vec![ "" ];

    assert_eq!(left_split_expected, left_vals);
    assert_eq!(right_split_expected, right_vals);
}

#[test]
fn rope_insert_lower_bounds(){
    let rope = create_example_rope();
    let rope = RopeNode::insert(rope.clone(), 0, "test".to_string()).unwrap();

    let expected = vec![ "test", "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_insert_first_letter(){
    let rope = create_example_rope();
    let rope = RopeNode::insert(rope.clone(), 11, "test".to_string()).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "test", "me_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_insert_mid_letter(){
    let rope = create_example_rope();
    let rope = RopeNode::insert(rope.clone(), 12, "test".to_string()).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "m", "test", "e_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_insert_last_letter(){
    let rope = create_example_rope();
    let rope = RopeNode::insert(rope.clone(), 15, "test".to_string()).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "test", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_insert_upper_bounds(){
    let rope = create_example_rope();
    let rope = RopeNode::insert(rope.clone(), 22, "test".to_string()).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon", "test" ];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_delete_lower_bound(){
    let rope = create_example_rope();
    let rope = RopeNode::delete(rope.clone(), 0, 6).unwrap();

    let expected = vec![ "my_", "na", "me_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_delete_first_letter(){
    let rope = create_example_rope();
    let rope = RopeNode::delete(rope.clone(), 11, 1).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "e_i", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_delete_mid_letter(){
    let rope = create_example_rope();
    let rope = RopeNode::delete(rope.clone(), 13, 2).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_delete_last_letter(){
    let rope = create_example_rope();
    let rope = RopeNode::delete(rope.clone(), 14, 1).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_", "s", "_Simon"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_delete_upper_bound(){
    let rope = create_example_rope();
    let rope = RopeNode::delete(rope.clone(), 20, 2).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Sim"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_delete_upper_bound_2(){
    let rope = create_example_rope();
    let rope = RopeNode::delete(rope.clone(), 21, 1).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simo"];
    let actual = RopeNode::get_leaves(rope.clone());

    println!("\nrope values: {:?}\n", actual);

    assert_eq!(expected, actual);
}

#[test]
fn rope_report(){
    let rope = create_example_rope();
    let report = RopeNode::report(rope.clone());
    assert_eq!("Hello_my_name_is_Simon", report);
}

