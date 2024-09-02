use crate::rope::{rope::Rope, rope_node::RopeNode};


/// Create a test rope for testing (https://en.wikipedia.org/wiki/Rope_(data_structure)#/media/File:Vector_Rope_example.svg)
fn create_example_rope() -> Rope {

    let mut rope = Rope::new();

    let mut a = RopeNode::new(); 
    let mut b = RopeNode::new(); 
    let mut c = RopeNode::new(); 
    let mut d = RopeNode::new(); 
    let mut e = RopeNode::new(); 
    let mut f = RopeNode::new(); 
    let mut g = RopeNode::new(); 
    let mut h = RopeNode::new(); 
    let mut j = RopeNode::new(); 
    let mut k = RopeNode::new(); 
    let mut m = RopeNode::new(); 
    let mut n = RopeNode::new(); 

    // set values, weights and make internal nodes
    a.weight = 22;
    b.weight = 9;
    c.weight = 6;
    d.weight = 6;
    e.weight = 6;
    f.weight = 3;
    g.weight = 2;
    h.weight = 1;
    j.weight = 2;
    k.weight = 4;
    m.weight = 1;
    n.weight = 6;

    e.value = Some("Hello_".to_string());
    f.value = Some("my_".to_string());
    j.value = Some("na".to_string());
    k.value = Some("me_i".to_string());
    m.value = Some("s".to_string());
    n.value = Some("_Simon".to_string());

    e.is_internal = false; 
    f.is_internal = false; 
    j.is_internal = false; 
    k.is_internal = false; 
    m.is_internal = false; 
    n.is_internal = false; 

    // make connections
    h.left = Some(Box::new(m));
    h.right = Some(Box::new(n));

    g.left = Some(Box::new(j));
    g.right = Some(Box::new(k));

    d.left = Some(Box::new(g));
    d.right = Some(Box::new(h));

    c.left = Some(Box::new(e));
    c.right = Some(Box::new(f));

    b.left = Some(Box::new(c));
    b.right = Some(Box::new(d));

    a.left = Some(Box::new(b));
    a.right = None;

    rope.root = Some(Box::new(a));

    rope
}


#[test]
fn creating_a_new_rope() {
    use crate::rope::rope::Rope;

    let rope = Rope::new();
    assert_eq!(rope.root, Some(Box::new(RopeNode::new())));
}

#[test]
fn creating_different_ropes() {
    use crate::rope::rope::Rope;

    let mut rope = Rope::new();
    let mut rope_2 = Rope::new();

    let test_node_1 = RopeNode {
        weight: 1,
        value: None,
        left: None,
        right: None,
        is_internal: true,
    };

    let test_node_2 = RopeNode{
        weight: 2,
        value: None,
        left: None,
        right: None,
        is_internal: true,
    };

    rope.root = Some(Box::new(test_node_1));
    rope_2.root = Some(Box::new(test_node_2));

    assert_ne!(rope, rope_2);
}

#[test]
fn rope_traversal(){

    let example_rope = create_example_rope();

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let actual = example_rope
        .iter()
        .map(|node| node.value.as_ref().unwrap())
        .collect::<Vec<&String>>();

    println!("\n{:?}\n", actual);
    assert_eq!(expected, actual);
}

#[test]
fn rope_concat_ideal(){

    use crate::rope::rope::Rope;

    let mut rope_1 = Rope::new();
    let mut rope_2 = Rope::new();

    let s1_1 = RopeNode {
        weight: 6,
        value: Some(String::from("Hello_")),
        left: None,
        right: None,
        is_internal: false,
    };

    let s1_2 = RopeNode {
        weight: 3,
        value: Some(String::from("my_")),
        left: None,
        right: None,
        is_internal: false,
    };

    let r1 = RopeNode {
        weight: 6,
        value: None,
        left: Some(Box::new(s1_1)),
        right: Some(Box::new(s1_2)),
        is_internal: true,
    };

    rope_1.root = Some(Box::new(r1));


    let s2_1 = RopeNode {
        weight: 2,
        value: Some(String::from("na")),
        left: None,
        right: None,
        is_internal: false,
    };

    let s2_2 = RopeNode {
        weight: 4,
        value: Some(String::from("me_i")),
        left: None,
        right: None,
        is_internal: false,
    };

    let r2 = RopeNode {
        weight: 2,
        value: None,
        left: Some(Box::new(s2_1)),
        right: Some(Box::new(s2_2)),
        is_internal: true,
    };

    rope_2.root = Some(Box::new(r2));

    let final_rope = Rope::concat(rope_1, rope_2).unwrap();

    let expected = vec![ "Hello_", "my_", "na", "me_i" ];
    let result = final_rope
        .iter()
        .map(|node| node.value.as_ref().unwrap())
        .collect::<Vec<&String>>();

    println!("\nconcat result: {:?}\n", result);

    assert_eq!(expected, result);
    assert_eq!(9, final_rope.root.as_ref().unwrap().weight);
}

#[test]
fn rope_max_depth(){
    let example_rope = create_example_rope();
    assert_eq!(5, example_rope.max_depth());
}

#[test]
fn rope_is_balanced_ideal(){

    let mut rope = Rope::new();
    let mut r1 = RopeNode::new();
    let n1 = RopeNode::new();
    let n2 = RopeNode::new();

    r1.left = Some(Box::new(n1));
    r1.right = Some(Box::new(n2));
    rope.root = Some(Box::new(r1));

    assert_eq!(true, RopeNode::is_balanced(&rope.root));
}

#[test]
fn rope_is_balanced_not_ideal(){

    let mut rope = Rope::new();
    let mut r1 = RopeNode::new();
    let mut n1 = RopeNode::new();
    let n2 = RopeNode::new();

    n1.left = Some(Box::new(n2));
    r1.left = Some(Box::new(n1));
    rope.root = Some(Box::new(r1));

    assert_eq!(false, RopeNode::is_balanced(&rope.root));
}

#[test]
fn rope_is_balanced_not_ideal_2(){

    let mut rope = Rope::new();
    let mut r1 = RopeNode::new();
    let mut n1 = RopeNode::new();
    let n2 = RopeNode::new();
    let mut n3 = RopeNode::new();
    let mut n4 = RopeNode::new();
    let n5 = RopeNode::new_leaf("leaf".to_string());

    n4.left = Some(Box::new(n5));
    n3.left = Some(Box::new(n4));
    n1.right = Some(Box::new(n3));

    r1.left = Some(Box::new(n1));
    r1.right = Some(Box::new(n2));
    rope.root = Some(Box::new(r1));

    assert_eq!(false, RopeNode::is_balanced(&rope.root));
}
