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

    pub fn concat(&self, rope_2: Rc<RefCell<RopeNode>>) -> Result<Rc<RefCell<RopeNode>>, Box<dyn std::error::Error>> {
        let new_root = RopeNode::new();
        let rope_1 = self.get_rc();

        let rope_weight = rope_1.borrow().get_length();

        new_root.borrow_mut().set_weight(rope_weight);
        new_root.borrow_mut().set_left(Rc::clone(&rope_1));
        new_root.borrow_mut().set_right(Rc::clone(&rope_2));

        //let new_rope = new_rope.rebalance();

        Ok(new_root)
    }

}
