use crate::rope::{rope::Rope, rope_node::RopeNode};

#[test]
fn rope_rebalance(){
    let rope = create_example_rope();
    let rope = rope.rebalance();
    assert_eq!(true, RopeNode::is_balanced(&rope.root));

    let expected = vec![ "Hello_", "my_", "na", "me_i", "s", "_Simon"];
    let actual = rope 
        .iter()
        .map(|node| node.value.as_ref().unwrap())
        .collect::<Vec<&String>>();

    assert_eq!(expected, actual);
}


#[test]
fn rope_index_of_ideal(){
    let rope = create_example_rope();
    let idx = 7;

    let result = rope.index_of(idx).unwrap();
    assert_eq!('y', result);
}

#[test]
fn rope_index_of_ideal_upper_bound(){
    let rope = create_example_rope();

    let values = rope 
        .iter()
        .map(|node| node.value.as_ref().unwrap().as_str())
        .collect::<Vec<&str>>()
        .join("");

    let idx = values.len() - 1;

    let result = rope.index_of(idx).unwrap();
    assert_eq!('n', result);
}

#[test]
fn rope_index_of_ideal_lower_bound(){
    let rope = create_example_rope();
    let idx = 0;

    let result = rope.index_of(idx).unwrap();
    assert_eq!('H', result);
}

#[test]
fn rope_index_of_out_of_bounds_large(){
    let rope = create_example_rope();
    //let idx = 8;
    let idx = 22;
    let result = rope.index_of(idx);

    assert_eq!(true, result.is_err());
}


#[test]
fn rope_split(){
    let rope = create_example_rope();

    let (left, right) = rope.split(12).unwrap();

    println!("\nleft: {:?}\n", left);
    println!("\nright: {:?}\n", right);
}
