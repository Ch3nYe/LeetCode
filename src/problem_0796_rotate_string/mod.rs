/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/7 14:10
*/
pub mod string_match;
pub mod string_concatenate;
// https://leetcode-cn.com/problems/rotate-string/
pub trait Solution {
    fn rotate_string(s: String, goal: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abcde", "cdeab", true),
            ("abcde", "abced", false),
        ];

        for (s, goal, expected) in test_cases {
            assert_eq!(S::rotate_string(s.to_string(),goal.to_string()), expected);
        }
    }
}