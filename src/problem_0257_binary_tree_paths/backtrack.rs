use std::borrow::Borrow;
use std::cell::RefCell;
use std::ops::Add;
use std::rc::Rc;
use crate::data_structures::TreeNode;

pub struct Solution;

impl Solution {
    fn binary_tree_paths_helper(node: &TreeNode, base: &mut String, result: &mut Vec<String>) {
        let traceback_len = base.len();

        base.push_str(format!("->{}", node.val).as_str());

        match (node.left.as_deref(), node.right.as_deref()) {
            (None, None) => result.push(base.clone()),
            (None, Some(child))|(Some(child), None) => Self::binary_tree_paths_helper(&child.borrow(), base, result),
            (Some(left), Some(right))=> {
                Self::binary_tree_paths_helper(&left.borrow(), base, result);
                Self::binary_tree_paths_helper(&right.borrow(), base, result);
            },
        }

        base.truncate(traceback_len);
    }

    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result = Vec::new();

        if let Some(node) = root.as_deref() {
            let node = node.borrow();
            let mut base = node.val.to_string();

            match (node.left.as_deref(), node.right.as_deref()) {
                (None, None) => result.push(base),
                (None, Some(child)) | (Some(child), None) => {
                    Self::binary_tree_paths_helper(&child.borrow(), &mut base, &mut result);
                }
                (Some(left), Some(right)) => {
                    Self::binary_tree_paths_helper(&left.borrow(), &mut base, &mut result);
                    Self::binary_tree_paths_helper(&right.borrow(), &mut base, &mut result);
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        Self::binary_tree_paths(root)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}