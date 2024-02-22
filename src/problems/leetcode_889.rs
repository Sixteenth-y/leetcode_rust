use std::{cell::RefCell, rc::Rc};

use crate::structure::tree_node::TreeNode;

use super::Solution;

impl Solution {
    /**
     * pre [root] [left] [right]
     * post [left] [right] [root]
     */
    pub fn construct_from_pre_post(preorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        if preorder.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode { val: preorder[0], left: None, right: None })));
        }

        let post_len = postorder.len();
        let left_size = postorder.iter().position(|&x| { x == preorder[1] }).unwrap() + 1;
        let pre1 = preorder[1 .. left_size + 1].to_vec();
        let pre2 = preorder[left_size + 1 ..].to_vec();
        let post1 = postorder[.. left_size].to_vec();
        let post2 = postorder[left_size .. post_len - 1].to_vec();
        let left = Self::construct_from_pre_post(pre1, post1);
        let right = Self::construct_from_pre_post(pre2, post2);
        Some(Rc::new(RefCell::new(TreeNode { val: postorder[post_len - 1], left, right, })))
    }
}