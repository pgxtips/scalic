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

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub fn is_internal(&self) -> bool {
        self.is_internal
    }

    pub fn get_length(node: &Option<Box<RopeNode>>) -> i32 {
        if node.is_none() { return 0; }
        let node = node.as_ref().unwrap();

        if node.is_leaf() {
            return node.value.as_ref().unwrap().len() as i32;
        } 

        let left_length = Self::get_length(&node.left);
        let right_length = Self::get_length(&node.right);

        return left_length + right_length; 
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

    pub fn index_of(&self, start_idx: usize) -> Result<char, Box<dyn std::error::Error>> {
        // to account for the fact the index used for the strings are 0 based
        let weight = self.weight as usize;

        //println!("start_idx: {}, weight: {}", start_idx, weight);

        // if less than the weight, then we are in the left subtree
        if start_idx <= weight {
            // if we are at a leaf node, then return the character
            if self.is_leaf() {
                let value = self.value.as_ref().unwrap();
                let char = value.chars().nth(start_idx);
                if char.is_none() { return Err("Index out of bounds".into()); }
                return Ok(char.unwrap());
            }

            let left = match self.left.as_ref(){
                Some(r) => r,
                None => return Err("Index out of bounds".into())
            };

            return left.index_of(start_idx);
        }
        // if greater than the weight, then we are in the right subtree
        if start_idx > weight {
            let right = match self.right.as_ref(){
                Some(r) => r,
                None => return Err("Index out of bounds".into())
            };
            return right.index_of(start_idx-weight);
        }
        
        Err("Error finding index".into())
    }



}
