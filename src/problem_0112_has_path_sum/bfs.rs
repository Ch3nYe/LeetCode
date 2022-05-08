use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::TreeNode;

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut stack = Vec::new();
        if root.is_none() { return false }
        stack.push((root.unwrap(), 0));
        while !stack.is_empty() {
            if let Some((node, sum)) = stack.pop() {
                let node = node.borrow();
                match (node.val, node.left.clone(), node.right.clone()) {
                    (val, None, None) => if sum+val==target_sum {return true},
                    (val, Some(child), None)|(val, None, Some(child)) => {stack.push((child, sum+val));}
                    (val, Some(left),Some(right)) => {stack.push((left, sum+val));stack.push((right, sum+val));}
                    _ => {}
                }
            }
        }
        false
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