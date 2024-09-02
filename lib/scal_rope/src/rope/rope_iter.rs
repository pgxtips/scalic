use crate::rope::rope::Rope;
use crate::rope::rope_node::RopeNode;

pub struct InOrderRopeIter<'a> {
    stack: Vec<&'a Box<RopeNode>>,
}

impl <'a> Iterator for InOrderRopeIter <'a>{
    type Item = &'a Box<RopeNode>;

    /// Provide a method to create an in-order iterator
    /// - returns an iterator over the leaves
    /// - non-consuming
    fn next(&mut self) -> Option<Self::Item> {
        self.next_leaf()
    }
}

impl<'a> InOrderRopeIter<'a> {
    pub fn new(rope: &'a Rope) -> Self {
        let mut stack = Vec::new();

        let mut current = rope.root.as_ref();

        while let Some(c) = current {
            stack.push(c);
            current = c.get_left().as_ref();
        }

        InOrderRopeIter {
            stack,
        }
    }

    pub fn next_leaf(&mut self) -> Option<&'a Box<RopeNode>> {

        let result = self.stack.pop();

        if !self.stack.is_empty(){
            let parent = self.stack.pop();
            let parent = match parent {
                Some(p) => p,
                None => return result,
            };

            let right = parent.get_right();

            match right {
                Some(r) => {
                    self.stack.push(r);
                    let mut cleft = r.get_left();

                    while cleft.is_some() {
                        self.stack.push(cleft.as_ref().unwrap());
                        cleft = cleft.as_ref().unwrap().get_left();
                    }
                },
                None => {},
            }
        }

        result
    }
}
