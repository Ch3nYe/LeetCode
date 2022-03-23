/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/22 22:34
*/
pub mod double_pointer;

pub trait Solution {
    fn valid_palindrome(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("aydmda", true),
            ("abc", false),
            ("aba", true),
            ("abca", true),
        ];

        for (s, expected) in test_cases {
            assert_eq!(S::valid_palindrome(s.to_string()), expected);
        }
    }
}