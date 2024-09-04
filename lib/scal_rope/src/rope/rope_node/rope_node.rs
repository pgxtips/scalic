use std::{cell::RefCell, rc::Rc};

use crate::rope::rope_node::rope_node_iter::InOrderRopeIter;


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

    pub fn get_weight(self_: Rc<RefCell<Self>>) -> usize {
        self_.borrow().weight.clone()
    }

    pub fn get_value(self_: Rc<RefCell<Self>>) -> Option<String> {
        self_.borrow().value.clone()
    }

    pub fn get_left(self_: Rc<RefCell<Self>>) -> Option<Rc<RefCell<RopeNode>>> {
        self_.borrow().left.clone()
    }

    pub fn get_right(self_: Rc<RefCell<Self>>) -> Option<Rc<RefCell<RopeNode>>> {
        self_.borrow().right.clone()
    }

    pub fn get_parent(self_: Rc<RefCell<Self>>) -> Option<Rc<RefCell<RopeNode>>> {
        self_.borrow().parent.clone()
    }

    // setters
    pub fn set_parent(self_: Rc<RefCell<Self>>, node: Rc<RefCell<RopeNode>>) {
        self_.borrow_mut().parent = Some(node);
    }
    
    pub fn set_weight(self_: Rc<RefCell<Self>>, weight: usize) {
        self_.borrow_mut().weight = weight;
    }

    pub fn set_left(self_: Rc<RefCell<Self>>, node: Rc<RefCell<RopeNode>>) {
        // set parent of the left node
        RopeNode::set_parent(Rc::clone(&node), Rc::clone(&self_));

        // set the left node of the current node
        self_.borrow_mut().left = Some(Rc::clone(&node));

        // casecade up the tree from the newly added node to 
        // update the weights of the parent nodes
        let mut start_parent = RopeNode::get_parent(Rc::clone(&node));
        while start_parent.is_some(){
            let parent = start_parent.unwrap();

            // get the left node of the parent
            let parent_left = RopeNode::get_left(Rc::clone(&parent));

            if parent_left.is_some() { 
                let parent_left = parent_left.unwrap();
                let new_parent_weight = RopeNode::get_length(Rc::clone(&parent_left));
                RopeNode::set_weight(Rc::clone(&parent), new_parent_weight);
            }

            start_parent = RopeNode::get_parent(Rc::clone(&parent));
        }
    }

    pub fn set_right(self_: Rc<RefCell<Self>>, node: Rc<RefCell<RopeNode>>) {
        // set parent of the right node
        RopeNode::set_parent(Rc::clone(&node), Rc::clone(&self_));
        // set the right node of the current node
        self_.borrow_mut().right = Some(Rc::clone(&node));

        // casecade up the tree from the newly added node to 
        // update the weights of the parent nodes
        let mut start_parent = RopeNode::get_parent(Rc::clone(&node));
        while start_parent.is_some(){
            let parent = start_parent.unwrap();

            // get the left node of the parent
            let parent_left = RopeNode::get_left(Rc::clone(&parent));

            if parent_left.is_some() { 
                let parent_left = parent_left.unwrap();
                let new_parent_weight = RopeNode::get_length(Rc::clone(&parent_left));
                RopeNode::set_weight(Rc::clone(&parent), new_parent_weight);
            }

            start_parent = RopeNode::get_parent(Rc::clone(&parent));
        }
    }

    /// Provide a method to create an in-order iterator
    /// - returns an iterator over the leaves
    pub fn iter(self_: Rc<RefCell<Self>>) -> InOrderRopeIter {
        InOrderRopeIter::new(Some(self_))
    }

    // methods 

    pub fn is_leaf(self_: Rc<RefCell<Self>>) -> bool {
        self_.borrow().left.is_none() && self_.borrow().right.is_none()
    }

    pub fn is_balanced(self_: Rc<RefCell<Self>>) -> bool {

        let left_depth = match RopeNode::get_left(Rc::clone(&self_)) {
            Some(left) => RopeNode::max_depth(Rc::clone(&left)),
            None => 0
        };
        let right_depth = match RopeNode::get_right(Rc::clone(&self_)) {
            Some(right) => RopeNode::max_depth(right),
            None => 0
        };

        let diff = (left_depth - right_depth).abs();

        let condition_1 = match RopeNode::get_left(Rc::clone(&self_)){
            Some(left) => RopeNode::is_balanced(Rc::clone(&left)),
            None => true
        };

        let condition_2 = match RopeNode::get_right(Rc::clone(&self_)) {
            Some(right) => RopeNode::is_balanced(Rc::clone(&right)),
            None => true
        };


        if diff <= 1 && condition_1 && condition_2 {
            return true;
        }

        return false;
    }

    pub fn get_leaves(self_: Rc<RefCell<Self>>) -> Vec<String> {
        let iter = RopeNode::iter(Rc::clone(&self_));
        let values = iter
            .filter(|node| RopeNode::is_leaf(Rc::clone(node)))
            .filter(|node| RopeNode::get_value(Rc::clone(node)).is_some())
            .map(|node| RopeNode::get_value(Rc::clone(&node)).unwrap())
            .collect::<Vec<String>>();
        values
    }

    pub fn max_depth(self_: Rc<RefCell<Self>>) -> i32 {

        let max_left = match RopeNode::get_left(Rc::clone(&self_)) {
            Some(node) => RopeNode::max_depth(Rc::clone(&node)),            
            None => 0
        };

        let max_right = match RopeNode::get_right(Rc::clone(&self_)) {
            Some(node) => RopeNode::max_depth(Rc::clone(&node)),            
            None => 0
        };

        return 1 + std::cmp::max(max_left, max_right); 
    }

    pub fn get_length(self_: Rc<RefCell<Self>>) -> usize {

        if RopeNode::is_leaf(Rc::clone(&self_)) {
            match RopeNode::get_value(Rc::clone(&self_)) {
                Some(value) => return value.len(),
                None => return 0
            }
        } 

        let left_length = match RopeNode::get_left(Rc::clone(&self_)){
            Some(node) => RopeNode::get_length(Rc::clone(&node)),
            None => 0
        };

        let right_length = match RopeNode::get_right(Rc::clone(&self_)) {
            Some(node) => RopeNode::get_length(Rc::clone(&node)),
            None => 0
        };

        return left_length + right_length; 
    }

    pub fn concat(self_: Rc<RefCell<Self>>, rope_2: Option<Rc<RefCell<RopeNode>>>) -> Result<Rc<RefCell<RopeNode>>, Box<dyn std::error::Error>> {

        if rope_2.is_none() { return Ok(self_); }
        let rope_2 = rope_2.unwrap();

        //println!("rope_1: {:?}", rope_1.borrow().get_leaves());
        //println!("rope_2: {:?}", rope_2.borrow().get_leaves());

        let new_root = RopeNode::new();

        let rope_weight = RopeNode::get_length(Rc::clone(&self_));
        RopeNode::set_left(Rc::clone(&new_root), Rc::clone(&self_));
        RopeNode::set_right(Rc::clone(&new_root), Rc::clone(&rope_2));
        RopeNode::set_weight(Rc::clone(&new_root), rope_weight);

        println!("new_rope weight: {:?}", RopeNode::get_weight(Rc::clone(&new_root)));

        //println!("new_rope: {:?}", new_root.borrow().get_leaves());
        //println!("new_rope_weight: {}", new_root.borrow().get_weight());

        Ok(new_root)
    }

    pub fn rebalance(self_: Rc<RefCell<Self>>) {

        if !RopeNode::is_balanced(Rc::clone(&self_)) {
            let leaves_iter = RopeNode::iter(Rc::clone(&self_));
            let leaves = leaves_iter.collect::<Vec<Rc<RefCell<RopeNode>>>>();
            let new_rope = Self::merge(&leaves, 0, leaves.len());

            let new_rope = match new_rope {
                Ok(rope) => rope,
                Err(e) => panic!("Error rebalancing rope: {}", e),
            };

            *self_.borrow_mut() = new_rope.borrow().clone();
        }
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

            let new_rope = RopeNode::concat(Rc::clone(&leaf), Some(Rc::clone(&leaf_2)))?;

            return Ok(new_rope);
        }

        let mid: usize = start + (range / 2);

        // return a new Rope with the merged left and right subtrees
        let left = Self::merge(leaves, start, mid)?;
        let right = Self::merge(leaves, mid, end)?;

        return Ok(RopeNode::concat(Rc::clone(&left), Some(Rc::clone(&right)))?);
    }

    pub fn index_of(self_: Rc<RefCell<Self>>, start_idx: usize) -> Result<char, Box<dyn std::error::Error>> {
        // to account for the fact the index used for the strings are 0 based
        let weight = RopeNode::get_weight(Rc::clone(&self_));

        // if less than the weight, then we are in the left subtree
        if start_idx <= weight {
            // if we are at a leaf node, then return the character
            if RopeNode::is_leaf(Rc::clone(&self_)) {
                let value = RopeNode::get_value(Rc::clone(&self_)).unwrap();
                let char = value.chars().nth(start_idx);
                if char.is_none() { return Err("Index out of bounds".into()); }
                return Ok(char.unwrap());
            }

            let left = match RopeNode::get_left(Rc::clone(&self_)) {
                Some(r) => r,
                None => return Err("Index out of bounds".into())
            };

            return RopeNode::index_of(Rc::clone(&left), start_idx);
        }
        // if greater than the weight, then we are in the right subtree
        if start_idx > weight {

            let right = match RopeNode::get_right(Rc::clone(&self_)){
                Some(r) => r,
                None => return Err("Index out of bounds".into())
            };

            return RopeNode::index_of(Rc::clone(&right), start_idx-weight);
        }

        Err("Error finding index".into())
    }

    /// Split the rope at the given index
    /// - returns a tuple of the left and right ropes
    /// - left rope contains the nodes from 0 to start_idx (original rope with split nodes
    /// removed)
    /// - right rope contains the nodes that were split off
    pub fn split(self_: Rc<RefCell<Self>>, start_idx: usize) -> Result<(Rc<RefCell<RopeNode>>, Rc<RefCell<RopeNode>>), Box<dyn std::error::Error>> {
        let weight = RopeNode::get_weight(Rc::clone(&self_));
        //println!("index: {}, weight: {}, length: {}", start_idx, weight, RopeNode::get_length(Rc::clone(&self_)));

        // bounds check
        if start_idx > RopeNode::get_length(Rc::clone(&self_)) {
            return Err("Index out of bounds".into()); 
        }

        // move to the left subtree
        if start_idx < weight { 

            // if we are at a leaf node, then split the node
            // and return the new left and new right nodes
            if RopeNode::is_leaf(Rc::clone(&self_)) {

                let chars_1 = RopeNode::get_value(Rc::clone(&self_))
                    .unwrap()
                    .chars()
                    .take(start_idx)
                    .collect::<String>();

                let chars_2 = RopeNode::get_value(Rc::clone(&self_))
                    .unwrap()
                    .chars()
                    .skip(start_idx)
                    .collect::<String>();

                let new_left_split_node = RopeNode::new_leaf(chars_1);
                let new_right_split_node = RopeNode::new_leaf(chars_2);

                return Ok((Rc::clone(&new_left_split_node), Rc::clone(&new_right_split_node)));
            }

            let next_node = RopeNode::get_left(Rc::clone(&self_)).unwrap();
            let (left_split, right_split) = RopeNode::split(Rc::clone(&next_node), start_idx)?;

            let return_left = left_split;
            RopeNode::rebalance(Rc::clone(&return_left));

            let self_right = RopeNode::get_right(Rc::clone(&self_));
            let return_right = RopeNode::concat(Rc::clone(&right_split), self_right)?;
            RopeNode::rebalance(Rc::clone(&return_right));

            return Ok((Rc::clone(&return_left), Rc::clone(&return_right)));
        } 
        // move to the right subtree
        else if start_idx > weight { 
            let next_node = RopeNode::get_right(Rc::clone(&self_)).unwrap();
            let (left_split, right_split) = RopeNode::split(Rc::clone(&next_node), start_idx-weight)?;

            let self_left = RopeNode::get_left(Rc::clone(&self_)).unwrap();
            let return_left = RopeNode::concat(Rc::clone(&self_left), Some(left_split))?;
            RopeNode::rebalance(Rc::clone(&return_left));

            let return_right = right_split;
            RopeNode::rebalance(Rc::clone(&return_right));

            return Ok((Rc::clone(&return_left), Rc::clone(&return_right)));
        }
        // if index as exactly at boundary
        else if start_idx == weight {

            let return_left = RopeNode::get_left(Rc::clone(&self_));
            let return_right = RopeNode::get_right(Rc::clone(&self_));

            if return_left.is_some() && return_right.is_some() {
                let return_left = return_left.unwrap();
                let return_right = return_right.unwrap();

                RopeNode::rebalance(Rc::clone(&return_left));
                RopeNode::rebalance(Rc::clone(&return_right));
                return Ok((Rc::clone(&return_left), Rc::clone(&return_right)));
            }
            else if return_left.is_none() {
                let empty_left = RopeNode::new_leaf("".into());
                return Ok((Rc::clone(&empty_left), Rc::clone(&return_right.unwrap())));
            } 
            else {
                let empty_right = RopeNode::new_leaf("".into());
                return Ok((Rc::clone(&return_left.unwrap()), Rc::clone(&empty_right)));
            } 
        }
        else {
            return Err("Error splitting rope".into());
        }

    }

    pub fn insert(self_: Rc<RefCell<Self>>, idx: usize, value: String) -> Result<Rc<RefCell<RopeNode>>, Box<dyn std::error::Error>> {

        // bound check
        if idx > RopeNode::get_length(Rc::clone(&self_)) {
            return Err("Index out of bounds".into());
        }

        let new_node = RopeNode::new_leaf(value);

        // if the rope is empty, then just insert the value
        if RopeNode::get_length(Rc::clone(&self_)) == 0 {
            let new_rope = RopeNode::concat(Rc::clone(&self_), Some(Rc::clone(&new_node)))?;
            //println!("new_rope: {:?}", RopeNode::get_leaves(Rc::clone(&new_rope)));
            RopeNode::rebalance(Rc::clone(&new_rope));
            return Ok(new_rope);
        }

        let (left_split, right_split) = match RopeNode::split(Rc::clone(&self_), idx){
            Ok((l, r)) => (l, r),
            Err(e) => panic!("Error splitting rope: {}", e),
        };


        let left_split_length = RopeNode::get_length(Rc::clone(&left_split));
        let right_split_length = RopeNode::get_length(Rc::clone(&right_split));

        if left_split_length == 0 { 
            let new_rope = RopeNode::concat(Rc::clone(&new_node), Some(Rc::clone(&right_split)))?;
            RopeNode::rebalance(Rc::clone(&new_rope));
            return Ok(new_rope); 
        }
        else if right_split_length == 0 { 
            let new_rope = RopeNode::concat(Rc::clone(&left_split), Some(Rc::clone(&new_node)))?;
            RopeNode::rebalance(Rc::clone(&new_rope));
            return Ok(new_rope); 
        }

        let left_split = RopeNode::concat(Rc::clone(&left_split), Some(Rc::clone(&new_node)))?;
        let new_rope = RopeNode::concat(Rc::clone(&left_split), Some(Rc::clone(&right_split)))?;
        RopeNode::rebalance(Rc::clone(&new_rope));

        Ok(new_rope)
    }

    pub fn delete(self_: Rc<RefCell<Self>>, start: usize, length: usize) -> Result<Rc<RefCell<RopeNode>>, Box<dyn std::error::Error>> {

        // bound check
        if start + length > RopeNode::get_length(Rc::clone(&self_)) {
            return Err("Index out of bounds".into());
        }

        //println!("self before deletion: {:?}\n", RopeNode::get_leaves(self_.clone()));

        let lhs = RopeNode::split(Rc::clone(&self_), start)?.0;
        //println!("self after lhs split: {:?}\n", RopeNode::get_leaves(self_.clone()));

        let rhs = RopeNode::split(Rc::clone(&self_), start+length)?.1;
        //println!("self after rhs split: {:?}", RopeNode::get_leaves(self_.clone()));

        let lhs_length = RopeNode::get_length(Rc::clone(&lhs));
        let rhs_length = RopeNode::get_length(Rc::clone(&rhs));

        if lhs_length == 0 { return Ok(rhs); }
        else if rhs_length == 0 { return Ok(lhs); }

        let result = RopeNode::concat(Rc::clone(&lhs), Some(Rc::clone(&rhs)))?;
        RopeNode::rebalance(Rc::clone(&result));

        return Ok(result);
    }

    pub fn report(self_: Rc<RefCell<Self>>) -> String{
        let leaves = RopeNode::get_leaves(Rc::clone(&self_)).join("");
        leaves
    }

}
