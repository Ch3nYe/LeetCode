/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/5/22 22:44
*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::TreeNode;

pub mod divide_conquer;
pub mod divide_conquer2;

pub trait Solution {
    fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::make_tree;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[Some(1),Some(2),Some(3),Some(4),Some(5),None,None] as &[_], 3),
            (&[Some(1),Some(2),None] as &[_], 1),
        ];

        for (root, expected) in test_cases {
            assert_eq!(S::diameter_of_binary_tree(
                make_tree(root.iter().copied())),
                       expected
            );
        }
    }
}