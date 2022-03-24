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
            (vec!["a".to_string(), "abc".to_string(), "bc".to_string(), "d".to_string()], "abc".to_string(), 3),
            (vec!["a".to_string(),"b".to_string(),"c".to_string()], "aaaaabbbbb".to_string(), 2),
            (vec!["a".to_string(),"a".to_string(),"a".to_string()], "ab".to_string(), 3),
        ];

        for (patterns,word, expected) in test_cases {
            assert_eq!(S::num_of_strings(patterns, word), expected);
        }
    }
}