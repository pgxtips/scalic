use std::fs;

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
fn rope_int_load_single_line_file_tiny(){
    let file_loc = "src/tests/test_data/single_line_tiny.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}

#[test]
fn rope_int_load_single_line_file_small(){
    let file_loc = "src/tests/test_data/single_line_small.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}

#[test]
fn rope_int_load_single_line_file_medium(){
    let file_loc = "src/tests/test_data/single_line_medium.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}

#[test]
fn rope_int_load_single_line_file_large(){
    let file_loc = "src/tests/test_data/single_line_large.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}

#[test]
fn rope_int_load_multi_line_file_small(){
    let file_loc = "src/tests/test_data/multi_line_small.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}

#[test]
fn rope_int_load_multi_line_file_medium(){
    let file_loc = "src/tests/test_data/multi_line_medium.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}

#[test]
fn rope_int_load_multi_line_file_large(){
    let file_loc = "src/tests/test_data/multi_line_large.txt";

    let mut rope = Rope::new();
    let _ = rope.load_file(file_loc);
    let loaded_file = rope.report();
    let expected = fs::read_to_string(file_loc).unwrap();

    assert_eq!(loaded_file, expected);
}
