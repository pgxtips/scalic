use std::{cell::RefCell, fs::File, io::{BufReader, Read}, path::Path, rc::Rc};

use crate::rope::rope_node::rope_node::RopeNode;

pub struct Rope {
    root: Rc<RefCell<RopeNode>>,
    size_limit: usize,
    chunk_size: usize,
}

impl Rope{
    pub fn new() -> Self {
        let root = RopeNode::new();
        Rope { 
            root, 
            size_limit: 1024 * 1024 * 15, // 15MB limit
            chunk_size: 1024,
        }
    }

    pub fn length(&self) -> usize {
        let root = self.root.clone();
        RopeNode::get_length(Rc::clone(&root))
    }

    pub fn char_at_index(&self, index: usize) -> char {
        let root = self.root.clone();
        let char = RopeNode::index_of(Rc::clone(&root), index);
        match char {
            Ok(res) => { res },
            Err(_e) => { ' ' },
        }
    }

    pub fn insert(&mut self, index: usize, value: String) {
        let root = Rc::clone(&self.root);
        match RopeNode::insert(Rc::clone(&root), index, value){
            Ok(res) => { self.root = res; },
            Err(_e) => {},
        };
    }

    pub fn report(&self) -> String {
        let root = self.root.clone();
        RopeNode::report(root)
    }

    pub fn delete(&mut self, index: usize, length: usize) {
        let root = self.root.clone();
        match RopeNode::delete(Rc::clone(&root), index, length){
            Ok(res) => { self.root = res; },
            Err(_e) => {},
        };
    }

    pub fn load_file(&mut self, path: &str) ->Result<(), Box<dyn std::error::Error>> {

        // validate path
        if !Path::new(path).exists(){ return Err("File could not be found".into()); }

        let chunk_size: usize = self.chunk_size;
        let root = Rc::clone(&self.root);

        let mut file = File::open(path)?;
        let mut buffer: Vec<u8> = vec![0; chunk_size];
        let mut limit = self.size_limit.clone();

        while limit > 0 {

            let bytes_read = file.read(&mut buffer[..])?;
            if bytes_read == 0 { break; }

            let str_value = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

            let new_node = RopeNode::new(); 
            let new_node = RopeNode::insert(Rc::clone(&new_node), 0, str_value.to_string())?;
            let new_node = RopeNode::concat(Rc::clone(&root), Some(Rc::clone(&new_node)))?;

            self.root = new_node;
            limit -= chunk_size; 
        }

        Ok(())
    }

}
