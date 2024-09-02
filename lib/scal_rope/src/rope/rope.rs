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

    pub fn get_length(&self) -> i32 {
        RopeNode::get_length(&self.root)
    }

    /// Concatenates two ropes together
    /// - consumes/moves the input ropes
    pub fn concat(rope_1: Rope, rope_2: Rope) -> Result<Rope, Box<dyn std::error::Error>> {
        let mut new_rope = Rope::new();
        let mut new_root = RopeNode::new();

        let rope_weight = rope_1.get_length();

        new_root.weight = rope_weight;
        new_root.left = rope_1.root;
        new_root.right = rope_2.root;

        new_rope.root = Some(Box::new(new_root));

        let new_rope = new_rope.rebalance();

        Ok(new_rope)
    }

    pub fn max_depth(&self) -> i32 {
        RopeNode::max_depth(&self.root)
    }

    pub fn is_balanced(&self) -> bool {
        RopeNode::is_balanced(&self.root)
    }

    /// Collect the set of leaves and rebuild the tree from the bottom-up.
    pub fn rebalance(mut self) -> Self {
        if !self.is_balanced() {
            let leaves = &self.iter().collect::<Vec<&Box<RopeNode>>>();
            let new_rope = Self::merge(&leaves, 0, leaves.len());

            match new_rope {
                Ok(rope) => self.root = rope.root,
                Err(e) => panic!("Error rebalancing rope: {}", e),
            };

        }

        return self;
    }

    /// Merge the leaves into a balanced tree
    ///
    /// This is a recursive function that will merge the leaves. It breaks
    /// down the leaves into smaller and smaller sets until it can merge in
    /// a binary fashion.
    ///
    /// Not efficient since we are cloning the nodes, however 
    /// I need to move on
    pub fn merge(leaves: &Vec<&Box<RopeNode>>, start: usize, end: usize) -> Result<Rope, Box<dyn std::error::Error>> {
        let range = end - start;

        //println!("start: {}, end: {}, range: {}", start, end, range);

        if range == 1 {
            let leaf = leaves.get(start).map(|l| (*l).clone());
            // return a new Rope with the leaf node
            let new_rope = Rope { root: leaf,};
            return Ok(new_rope);
        }
        else if range == 2 {

            let leaf = leaves.get(start).map(|l| (*l).clone());
            let leaf_2 = leaves.get(start+1).map(|l| (*l).clone());

            let new_rope = Rope::concat(
                Rope { root: leaf, },
                Rope { root: leaf_2, },
            )?;

            return Ok(new_rope);
        }

        let mid: i32 = start as i32 + (range as i32 / 2);
        let mid: usize = mid as usize;

        // return a new Rope with the merged left and right subtrees
        let left = Self::merge(leaves, start, mid);
        let right = Self::merge(leaves, mid, end);

        return Ok(Rope::concat(left?, right?)?);
    }

    /// index_of(i): return the character at position i
    /// retrieves the i-th character, via a 
    /// recursive search from the root node
    pub fn index_of(&self, index: usize) -> Result<char, Box<dyn std::error::Error>> {
        let root_node = self.root.as_ref().unwrap();
        root_node.index_of(index)
    }

    pub fn split(self, index: usize) -> Result<(Rope, Rope), Box<dyn std::error::Error>>{
        self.split_2(index)
    }

    fn split_2(self, index: usize) -> Result<(Rope, Rope), Box<dyn std::error::Error>> {

        let current_node = match self.root.as_ref(){
            Some(node) => node,
            None => return Err("No root node".into()),
        };

        let left = match &current_node.left{
            Some(node) => Some(Rope { root: Some(node.clone()) }),
            None => None,
        };

        let right = match &current_node.right{
            Some(node) => Some(Rope { root: Some(node.clone()) }),
            None => None,
        };

        let weight = current_node.weight as usize;

        if index < weight {

            println!("\n{:?}\n", current_node);

            let left = match left{
                Some(r) => r,
                None => return Err("No left node".into()),
            };
            
            let split = match left.split(index){
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            let new_left = split.0.rebalance();

            let new_right = match right{
                Some(r) => Rope::concat(split.1, r)?,
                None => Rope::new(),
            };

            return Ok((new_left, new_right));
        } else if index > weight {

            let right = match right{
                Some(r) => r,
                None => return Err("No right node".into()),
            };

            let split = match right.split(index-weight){
                Ok(s) => s,
                Err(e) => return Err(e),
            };

            let new_left = match left{
                Some(l) => Rope::concat(l, split.0)?,
                None => Rope::new(),
            };

            let new_right = split.1.rebalance();

            return Ok((new_left, new_right));
        } else {
            let left = match left{
                Some(r) => r,
                None => Rope::new(),
            };
            let right = match right{
                Some(r) => r,
                None => Rope::new(),
            };
            return Ok((left, right));
        }
    }


}
