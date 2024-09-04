use std::{cell::RefCell, rc::Rc};

use crate::rope::rope_node::rope_node::RopeNode;

pub struct Rope {
    root: Rc<RefCell<RopeNode>>,
}

impl Rope{
    pub fn new() -> Self {
        let root = RopeNode::new();
        Rope { root }
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

}
