/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/24 15:33
*/

pub mod bruteforce;

pub trait Solution {
    fn num_of_strings(patterns: Vec<String>, word: String) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (vec!["a", "abc", "bc", "d"].into_iter().map(|x| x.to_string()).collect(), "abc", 3),
            (vec!["a","b","c"].into_iter().map(|x| x.to_string()).collect(), "aaaaabbbbb", 2),
            (vec!["a","a","a"].into_iter().map(|x| x.to_string()).collect(), "ab", 3),
        ];

        for (patterns,word, expected) in test_cases {
            assert_eq!(S::num_of_strings(patterns, word.to_string()), expected);
        }
    }
}