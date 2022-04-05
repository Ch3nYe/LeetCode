/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/5 15:25
*/
pub mod dynamic;

pub trait Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abcde", "ace", 3),
            ("abc", "abc", 3),
            ("abc", "def", 0),
        ];

        for (text1, text2, expected) in test_cases {
            assert_eq!(S::longest_common_subsequence(text1.to_string(),text2.to_string()), expected);
        }
    }
}