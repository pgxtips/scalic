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

    /// Provide a method to create an in-order iterator
    /// - returns an iterator over the leaves
    pub fn iter<'a>(&'a self) -> InOrderRopeIter<'a> {
        InOrderRopeIter::new(self)
    }

    /// Concatenates two ropes together
    /// - consumes/moves the input ropes
    pub fn concat(rope_1: Rope, rope_2: Rope) -> Result<Rope, Box<dyn std::error::Error>> {
        let mut new_rope = Rope::new();
        let mut new_root = RopeNode::new();

        let new_weight = match rope_1.root.as_ref() {
            Some(node) => node.get_left_weight()? + node.get_right_weight()?,
            None => Err("Rope 1 has no root")?,
        };

        new_root.weight = new_weight;
        new_root.left = rope_1.root;
        new_root.right = rope_2.root;

        new_rope.root = Some(Box::new(new_root));

        Ok(new_rope)
    }

    pub fn max_depth(&self) -> i32 {
        RopeNode::max_depth(&self.root)
    }

}
