use crate::rope::{rope::Rope, rope_node::RopeNode};


fn create_test_rope() -> Rope {

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

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let mut actual = Vec::new();

    let example_rope = create_test_rope();

    for node in example_rope.iter() {
        match &node.value {
            Some(v) => actual.push(v),
            None => {},
        }
    }
    
    println!("\n{:?}\n", actual);
    assert_eq!(expected, actual);
}
