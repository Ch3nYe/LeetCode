/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/17 14:30
*/
//https://leetcode-cn.com/problems/non-overlapping-intervals/
pub mod greedy;

pub trait Solution {
    fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1,2],[2,3],[3,4],[1,3]] as &[[_]], 1),
            (&[ [1,2], [1,2], [1,2] ], 2),
            (&[ [1,2], [2,3] ], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::erase_overlap_intervals(nums.to_vec()), expected);
        }
    }
}