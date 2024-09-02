#[derive(Debug, Clone, PartialEq)]
pub struct RopeNode{
    pub weight: i32,
    pub value: Option<String>,
    pub left: Option<Box<RopeNode>>,
    pub right: Option<Box<RopeNode>>,
    pub is_internal: bool,
}

impl RopeNode {
    /// defaults to an internal node
    pub fn new() -> Self {
        RopeNode {
            weight: 0,
            value: None,
            left: None,
            right: None,
            is_internal: true,
        }
    }

    pub fn get_left(&self) -> &Option<Box<RopeNode>> {
        &self.left
    }

    pub fn get_left_weight(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let left = &self.left;
        match left {
            Some(node) => Ok(node.weight),
            None => Err("No left node exists")?,
        }
    }

    pub fn get_right(&self) -> &Option<Box<RopeNode>> {
        &self.right
    }

    pub fn get_right_weight(&self) -> Result<i32, Box<dyn std::error::Error>> {
        let right = &self.right;
        match right {
            Some(node) => Ok(node.weight),
            None => Err("No right node exists")?,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn is_internal(&self) -> bool {
        self.is_internal
    }
}
