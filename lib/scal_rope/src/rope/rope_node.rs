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

    pub fn new_leaf(value: String) -> Self {
        RopeNode {
            weight: 0,
            value: Some(value),
            left: None,
            right: None,
            is_internal: false,
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

    pub fn max_depth(node: &Option<Box<RopeNode>>) -> i32 {
        if node.is_none() { return 0; }

        let node = node.as_ref().unwrap();

        let max_left = Self::max_depth(&node.left);
        let max_right = Self::max_depth(&node.right);

        return 1 + std::cmp::max(max_left, max_right); 
    }

    pub fn is_balanced(node: &Option<Box<RopeNode>>) -> bool {
        if node.is_none() { return true; }
        let node = node.as_ref().unwrap();

        let left_depth = Self::max_depth(&node.left);
        let right_depth = Self::max_depth(&node.right);
        let diff = (left_depth - right_depth).abs();

        if diff <= 1 && Self::is_balanced(&node.left) && Self::is_balanced(&node.right)  {
            return true;
        }

        false 
    }
}
