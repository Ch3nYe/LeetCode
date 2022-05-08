use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::TreeNode;

pub struct Solution;

impl Solution {
    pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, curr_sum: i32, stack: &mut Vec<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return false;
        }
        if let Some(mut node) = root {
            let (val, left, right) = {
                let node_ref = node.borrow();

                (node_ref.val, node_ref.left.clone(), node_ref.right.clone())
            };
            if val + curr_sum == target_sum {
                return true;
            } else {

            }
        }
        false

    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = Vev::new();
        Self::dfs(root, target_sum, 0, &mut stack)

    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        Self::has_path_sum(root, sum)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}