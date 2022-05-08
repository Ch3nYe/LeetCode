use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::TreeNode;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let n = node.borrow();
                match (n.val, n.left.clone(), n.right.clone()) {
                    (val, None, None) => target_sum==val, // 递归终止条件，约束
                    (val, child, None)|(val, None, child) => {
                        if target_sum<val { false } else { // 剪枝
                            Self::has_path_sum(child, target_sum-val)
                        }
                    },
                    (val, left, right) => {
                        if target_sum<val { false } else {  // 剪枝
                            Self::has_path_sum(left,target_sum-val)|Self::has_path_sum(right,target_sum-val)
                        }
                    },
                    _ => false
                }
            },
            None => false
        }

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