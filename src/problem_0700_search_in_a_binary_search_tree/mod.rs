/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/6/2 14:54
*/
use std::cell::RefCell;
use std::rc::Rc;
use crate::data_structures::TreeNode;

pub mod iterative;

pub trait Solution {
    fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                (&[Some(4), Some(2), Some(7), Some(1), Some(3)] as &[_], 2),
                &[Some(2), Some(1), Some(3)] as &[_],
            ),

            (
                (&[Some(4), Some(2), Some(7), Some(1), Some(3)], 5),
                &[]
            ),
        ];

        for ((root, val), expected) in test_cases {
            assert_eq!(
                S::search_bst(test_utilities::make_tree(root.iter().copied()), val),
                test_utilities::make_tree(expected.iter().copied())
            )
        }
    }

}
