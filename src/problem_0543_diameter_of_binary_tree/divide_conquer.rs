pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::TreeNode;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {

        Self::dfs(root)
    }

    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let depth_left = Self::dfs(node.left.clone());
            let depth_right = Self::dfs(node.right.clone());
            return depth_left.max(depth_right)+1;
        } else {
            return 0;
        }

    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::diameter_of_binary_tree(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}