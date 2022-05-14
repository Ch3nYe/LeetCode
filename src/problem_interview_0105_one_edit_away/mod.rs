/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/5/13 10:51
*/
pub mod solve;

pub trait Solution {
    fn one_edit_away(first: String, second: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("", "a", true),

            ("pale", "ple", true),
            ("pales", "pal", false),
        ];

        for (first, second, expected) in test_cases {
            assert_eq!(S::one_edit_away(first.to_string(),second.to_string()), expected);
        }
    }
}