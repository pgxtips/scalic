#[allow(unused_imports)]
use crate::Rope;

#[test]
fn rope_int_insert(){
    let mut rope = Rope::new();
    rope.insert(0, "Hello".to_string());
    assert_eq!(rope.report(), "Hello".to_string());
}

#[test]
fn rope_int_insert_2(){
    let mut rope = Rope::new();
    rope.insert(0, "Hello".to_string());
    rope.insert(2, "x".to_string());
    assert_eq!(rope.report(), "Hexllo".to_string());
}

#[test]
fn rope_int_delete(){
    let mut rope = Rope::new();
    rope.insert(0, "Hello".to_string());
    rope.delete(2, 1);
    assert_eq!(rope.report(), "Helo".to_string());
}

#[test]
fn rope_int_load_single_line_file_small(){
    let mut rope = Rope::new();
    let _ = rope.load_file("src/tests/test_data/single_line_tiny.txt");

    todo!("write loads of tests for rope node weight updating");

    //println!("{}", rope.char_at_index(0));

    println!("{}", rope.report());
    println!("{}", rope.length());
}
