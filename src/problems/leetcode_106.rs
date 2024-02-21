use std::{cell::RefCell, rc::Rc};

use crate::{structure::tree_node::TreeNode, Solution};

impl Solution {
    pub fn build_tree_leetcode_106(
        inorder: Vec<i32>,
        postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() {
            return None;
        }
        let len = postorder.len();
        let left_size = inorder
            .iter()
            .position(|&x| x == postorder[len - 1])
            .unwrap();
        let post1 = postorder[0..left_size].to_vec();
        let post2 = postorder[left_size..len - 1].to_vec();
        let in1 = inorder[..left_size].to_vec();
        let in2 = inorder[left_size + 1..].to_vec();
        let left = Self::build_tree(in1, post1);
        let right = Self::build_tree(in2, post2);
        Some(Rc::new(RefCell::new(TreeNode {
            val: postorder[len - 1],
            left,
            right,
        })))
    }
}
