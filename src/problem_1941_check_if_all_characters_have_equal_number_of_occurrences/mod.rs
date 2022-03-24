/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/20 19:22
*/

pub mod hashmap;

pub trait Solution {
    fn are_occurrences_equal(s: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("abacbc", true),
            ("aaabb", false),
            ("aaabbb", true),
        ];

        for (str, expected) in test_cases {
            assert_eq!(S::are_occurrences_equal(str.to_string()), expected);
        }
    }
}

