use std::{cell::RefCell, rc::Rc};

use crate::rope::rope_iter_new::InOrderRopeIter;


#[derive(Debug, Clone, PartialEq)]
pub struct RopeNode {
    parent: Option<Rc<RefCell<RopeNode>>>,
    weight: usize,
    value: Option<String>,
    left: Option<Rc<RefCell<RopeNode>>>,
    right: Option<Rc<RefCell<RopeNode>>>,
}

impl RopeNode {

    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(RopeNode {
            parent: None,
            weight: 0,
            value: None,
            left: None,
            right: None,
        }))
    }

    pub fn new_leaf(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(RopeNode {
            parent: None,
            weight: value.len(),
            value: Some(value),
            left: None,
            right: None,
        }))
    }

    // getters 
    fn get_rc(&self) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(self.clone()))
    }

    pub fn get_weight(&self) -> usize {
        self.weight
    }

    pub fn get_value(&self) -> Option<String> {
        self.value.clone()
    }

    pub fn get_left(&self) -> Option<Rc<RefCell<RopeNode>>> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Option<Rc<RefCell<RopeNode>>> {
        self.right.clone()
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<RopeNode>>> {
        self.parent.clone()
    }

    // setters
    pub fn set_left(&mut self, node: Rc<RefCell<RopeNode>>) {
        node.borrow_mut().parent = Some(Rc::clone(&self.get_rc()));
        self.left = Some(node);
    }

    pub fn set_left_none(&mut self) {
        self.left = None;
    }

    pub fn set_right(&mut self, node: Rc<RefCell<RopeNode>>) {
        node.borrow_mut().parent = Some(Rc::clone(&self.get_rc()));
        self.right = Some(node);
    }

    pub fn set_right_none(&mut self) {
        self.right= None;
    }

    pub fn set_weight(&mut self, weight: usize) {
        self.weight = weight;
    }

    /// Provide a method to create an in-order iterator
    /// - returns an iterator over the leaves
    pub fn iter(&self) -> InOrderRopeIter {
        InOrderRopeIter::new(Some(self.get_rc()))
    }

    // methods 
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn is_balanced(&self) -> bool {

        let node_rc = self.get_rc();
        let node = node_rc.borrow();

        let left_depth = match node.get_left() {
            Some(left) => left.borrow().max_depth(),
            None => 0
        };
        let right_depth = match node.get_right() {
            Some(right) => right.borrow().max_depth(),
            None => 0
        };

        let diff = (left_depth - right_depth).abs();

        let condition_1 = match node.get_left(){
            Some(left) => left.borrow().is_balanced(),
            None => true
        };

        let condition_2 = match node.get_right(){
            Some(right) => right.borrow().is_balanced(),
            None => true
        };


        if diff <= 1 && condition_1 && condition_2 {
            return true;
        }

        return false;
    }

    pub fn print_leaves(&self) {
        let leaves = self.iter().collect::<Vec<Rc<RefCell<RopeNode>>>>();
        for leaf in leaves {
            println!("leaf: {:?}", leaf.borrow().get_value());
        }
    }

    pub fn max_depth(&self) -> i32 {

        let max_left = match &self.left{
            Some(node) => node.borrow().max_depth(),            
            None => 0
        };

        let max_right = match &self.right {
            Some(node) => node.borrow().max_depth(),            
            None => 0
        };

        return 1 + std::cmp::max(max_left, max_right); 
    }

    pub fn get_length(&self) -> usize {


        if self.is_leaf() {
            return self.get_value().unwrap().len();
        } 

        let left_length = match self.get_left(){
            Some(node) => node.borrow().get_length(),
            None => 0
        };
        let right_length = match self.get_right() {
            Some(node) => node.borrow().get_length(),
            None => 0
        };

        return left_length + right_length; 
    }

    pub fn concat(&self, rope_2: Option<Rc<RefCell<RopeNode>>>) -> Result<Rc<RefCell<RopeNode>>, Box<dyn std::error::Error>> {
        let new_root = RopeNode::new();
        let rope_1 = self.get_rc();

        if rope_2.is_none() { return Ok(rope_1); }

        let rope_weight = rope_1.borrow().get_length();
        new_root.borrow_mut().set_weight(rope_weight);
        new_root.borrow_mut().set_left(Rc::clone(&rope_1));
        new_root.borrow_mut().set_right(Rc::clone(&rope_2.unwrap()));

        //let new_rope = new_rope.rebalance();

        Ok(new_root)
    }

    pub fn rebalance(&self) -> Rc<RefCell<RopeNode>> {

        if !self.is_balanced() {
            let leaves = self.iter().collect::<Vec<Rc<RefCell<RopeNode>>>>();
            let new_rope = Self::merge(&leaves, 0, leaves.len());

            let new_rope = match new_rope {
                Ok(rope) => rope,
                Err(e) => panic!("Error rebalancing rope: {}", e),
            };

            return new_rope;
        }

        return self.get_rc();
    }

    /// Merge the leaves into a balanced tree
    ///
    /// This is a recursive function that will merge the leaves. It breaks
    /// down the leaves into smaller and smaller sets until it can merge in
    /// a binary fashion.
    ///
    /// Not efficient since we are cloning the nodes, however 
    /// I need to move on
    fn merge(leaves: &Vec<Rc<RefCell<RopeNode>>>, start: usize, end: usize) -> Result<Rc<RefCell<RopeNode>>, Box<dyn std::error::Error>> {
        let range = end - start;

        //println!("start: {}, end: {}, range: {}", start, end, range);

        if range == 1 {
            let leaf = match leaves.get(start).map(|l| (*l).clone()){
                Some(l) => l,
                None => return Err("No leaf found".into()),
            };
            return Ok(leaf);
        }
        else if range == 2 {

            let leaf = match leaves.get(start).map(|l| (*l).clone()){
                Some(l) => l,
                None => return Err("No leaf found".into()),
            };
            let leaf_2 = match leaves.get(start+1).map(|l| (*l).clone()){
                Some(l) => l,
                None => return Err("No leaf_2 found".into()),
            };

            let new_rope = leaf.borrow().concat(Some(leaf_2))?; 

            return Ok(new_rope);
        }

        let mid: i32 = start as i32 + (range as i32 / 2);
        let mid: usize = mid as usize;

        // return a new Rope with the merged left and right subtrees
        let left = Self::merge(leaves, start, mid)?;
        let right = Self::merge(leaves, mid, end)?;

        return Ok(left.borrow().concat(Some(right))?);
    }

    pub fn index_of(&self, start_idx: usize) -> Result<char, Box<dyn std::error::Error>> {
        // to account for the fact the index used for the strings are 0 based
        let weight = self.get_weight();

        //println!("start_idx: {}, weight: {}", start_idx, weight);

        // if less than the weight, then we are in the left subtree
        if start_idx <= weight {
            // if we are at a leaf node, then return the character
            if self.is_leaf() {
                let value = self.get_value().unwrap();
                let char = value.chars().nth(start_idx);
                if char.is_none() { return Err("Index out of bounds".into()); }
                return Ok(char.unwrap());
            }

            let left = match &self.left {
                Some(r) => r,
                None => return Err("Index out of bounds".into())
            };

            return left.borrow().index_of(start_idx);
        }
        // if greater than the weight, then we are in the right subtree
        if start_idx > weight {

            let right = match &self.right {
                Some(r) => r,
                None => return Err("Index out of bounds".into())
            };

            return right.borrow().index_of(start_idx-weight);
        }

        Err("Error finding index".into())
    }

    /// Split the rope at the given index
    /// - returns a tuple of the left and right ropes
    /// - left rope contains the nodes from 0 to start_idx (original rope with split nodes
    /// removed)
    /// - right rope contains the nodes that were split off
    pub fn split(&mut self, start_idx: usize) -> Result<(Rc<RefCell<RopeNode>>, Rc<RefCell<RopeNode>>), Box<dyn std::error::Error>> {
        let weight = self.get_weight();
        //println!("index: {}, weight: {}, length: {}", start_idx, weight, self.get_length());

        // bounds check
        if start_idx > self.get_length() { return Err("Index out of bounds".into()); }

        let current_node = self.get_rc();
        // move to the left subtree
        if start_idx < weight { 

            // if we are at a leaf node, then split the node
            // and return the left and right nodes
            // this is so that we dont try and get the left node and cause a panic
            if current_node.borrow().is_leaf() {

                let parent = current_node.borrow().get_parent().unwrap();
                let copy_current = Rc::clone(&current_node);

                let chars_1 = copy_current.borrow()
                    .get_value()
                    .unwrap()
                    .chars()
                    .take(start_idx)
                    .collect::<String>();

                let chars_2 = copy_current.borrow()
                    .get_value()
                    .unwrap()
                    .chars()
                    .skip(start_idx)
                    .collect::<String>();

                let new_left_node = RopeNode::new_leaf(chars_1);
                let new_right_node = RopeNode::new_leaf(chars_2);

                parent.borrow_mut().set_left(new_left_node);

                let return_left = Rc::clone(&parent);
                let return_right = new_right_node;

                return Ok((return_left, return_right));
            }

            let next_node = current_node.borrow().get_left().unwrap();
            let (left_split, right_split) = next_node.borrow_mut().split(start_idx)?;

            let return_left = left_split.borrow().rebalance();
            let return_right = right_split.borrow().concat(current_node.borrow().get_right())?;

            return Ok((return_left, return_right));
        } 
        // move to the right subtree
        else if start_idx > weight { 
            let next_node = current_node.borrow().get_right().unwrap();
            let (left_split, right_split) = next_node.borrow_mut().split(start_idx-weight)?;

            let return_left = current_node.borrow().get_left().unwrap().borrow().concat(Some(left_split))?;
            let return_right = right_split.borrow().rebalance();

            return Ok((return_left, return_right));
        }
        // if index as exactly at boundary
        else if start_idx == weight {
            //println!("splitting at boundary");
            let right_node = current_node.borrow().get_right().unwrap();
            return Ok((current_node.borrow().get_left().unwrap(), right_node));
        }
        else {
            return Err("Error splitting rope".into());
        }

    }

}
