/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/14 16:43
*/
// pub mod dynamic; // n^2
pub mod dynamic_opt; // nlogn

//https://leetcode-cn.com/problems/longest-increasing-subsequence/submissions/
pub trait Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3,5,6,2,5,4,19,5,6,7,12] as &[_], 6),
            (&[10,9,2,5,3,7,101,18] as &[_], 4),
            (&[1,2,3,0,4,5], 5),
            (&[0,1,0,3,2,3], 4),
            (&[7,7,7,7,7,7,7], 1),
            (&[0],1)
        ];

        for (nums , expected) in test_cases {
            assert_eq!(S::length_of_lis(nums.to_vec()), expected);
        }
    }
}