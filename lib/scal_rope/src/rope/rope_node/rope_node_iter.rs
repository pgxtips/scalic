use std::{cell::RefCell, rc::Rc};

use crate::rope::rope_node::rope_node::RopeNode;

pub struct InOrderRopeIter {
    stack: Vec<Rc<RefCell<RopeNode>>>,
}

impl Iterator for InOrderRopeIter {
    type Item = Rc<RefCell<RopeNode>>;

    /// Provide a method to create an in-order iterator
    /// - returns an iterator over the leaves
    /// - non-consuming
    fn next(&mut self) -> Option<Self::Item> {
        self.next_leaf()
    }
}

impl InOrderRopeIter {
    pub fn new(node: Option<Rc<RefCell<RopeNode>>>) -> Self {
        let mut stack = Vec::new();
        let mut node = node;

        while let Some(c) = node {
            stack.push(Rc::clone(&c));
            node = RopeNode::get_left(c);
        }

        InOrderRopeIter {
            stack,
        }
    }

    pub fn next_leaf(&mut self) -> Option<Rc<RefCell<RopeNode>>>{

        let result = self.stack.pop();

        if !self.stack.is_empty(){
            let parent = self.stack.pop();
            let parent = match parent {
                Some(p) => p,
                None => return result,
            };

            let right = RopeNode::get_right(parent);

            match right {
                Some(r) => {
                    self.stack.push(Rc::clone(&r));
                    let mut cleft = RopeNode::get_left(r);

                    while cleft.is_some() {
                        let val = cleft.unwrap();
                        self.stack.push(Rc::clone(&val));
                        cleft = RopeNode::get_left(val);
                    }
                },
                None => {},
            }
        }

        result
    }
}
