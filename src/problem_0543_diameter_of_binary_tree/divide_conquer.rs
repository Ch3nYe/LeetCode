pub struct Solution{
    res: i32
}

use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::TreeNode;

impl Solution {

    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut s = Solution{ res: 0 };
        s.dfs(root);
        s.res-1
    }

    fn dfs(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let node = node.borrow();
            let depth_left = Self::dfs(self,node.left.clone());
            let depth_right = Self::dfs(self,node.right.clone());
            self.res = self.res.max(depth_right+depth_left+1);
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