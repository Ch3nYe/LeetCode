/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/5/30 13:14
*/
pub mod bfs;
// https://leetcode.cn/problems/jump-game-ii/comments/
pub trait Solution {
    fn jump(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(&[2, 3, 1, 1, 4] as &[_], 2)];

        for (nums, expected) in test_cases {
            assert_eq!(S::jump(nums.to_vec()), expected);
        }
    }
}