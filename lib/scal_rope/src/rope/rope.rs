use crate::rope::rope_node::RopeNode;
use crate::rope::rope_iter::InOrderRopeIter;

#[derive(Debug, PartialEq)]
pub struct Rope {
    pub root: Option<Box<RopeNode>>,
}

impl Rope {
    pub fn new() -> Self {
        Rope {
            root: Some(Box::new(RopeNode::new())),
        }
    }

    // Provide a method to create an in-order iterator
    pub fn iter<'a>(&'a self) -> InOrderRopeIter<'a> {
        InOrderRopeIter::new(self)
    }
}
