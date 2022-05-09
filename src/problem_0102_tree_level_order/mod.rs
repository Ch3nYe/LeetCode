/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/5/9 21:00
*/
pub mod bfs;

use std::rc::Rc;
use std::cell::RefCell;
use crate::data_structures::TreeNode;

pub trait Solution {
    fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use crate::test_utilities::make_tree;
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)] as &[_],
              &[&[3] as &[_], &[9, 20], &[15, 7]] as &[&[_]]),
            (&[Some(1)], &[&[1]]),
            (&[], &[]),
        ];

        for (root, expected) in test_cases {
            assert_eq!(S::level_order(
                make_tree(root.iter().copied())),
                       expected
            );
        }
    }
}