use core::borrow;
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        
        let left = match &self.left {
            Some(it) => it.borrow().val.to_string(),
            None => String::from("null"),
        };

        let right = match &self.right {
            Some(it) => it.borrow().val.to_string(),
            None => String::from("null"),
        };
        write!(f, "[{}, {}, {}]", self.val, &left, &right)
    }
}
