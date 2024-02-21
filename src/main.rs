use crate::problems::Solution;

pub mod problems;
pub mod structure;

fn main() {
    println!("Hello, world!");
    let preorder = vec![3,9,20,15,7];
    let inorder = vec![9,3,15,20,7];
    let ans = Solution::build_tree(preorder, inorder);

    println!("{:?}", ans)
}
