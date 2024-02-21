use std::{cell::RefCell, rc::Rc};

use crate::{structure::tree_node::TreeNode, Solution};

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let left_size = inorder.iter().position(|&x| x == preorder[0]).unwrap();
        let pre1 = preorder[1..=left_size].to_vec();
        let pre2 = preorder[1 + left_size..].to_vec();
        let in1 = inorder[..left_size].to_vec();
        let in2 = inorder[left_size + 1..].to_vec();
        let left = Self::build_tree(pre1, in1);
        let right = Self::build_tree(pre2, in2);
        Some(Rc::new(RefCell::new(TreeNode {
            val: preorder[0],
            left,
            right,
        })))
    }
}
