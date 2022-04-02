/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/1 20:52
*/
pub mod hashmap_slide_window;

pub trait Solution {
    fn check_inclusion(s1: String, s2: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("adc", "dcda", true),
            ("ab", "eidbaooo", true),
            ("ab", "eidboaoo", false),
        ];

        for (s1, s2, expected) in test_cases {
            assert_eq!(S::check_inclusion(s1.to_string(),s2.to_string()), expected);
        }
    }
}