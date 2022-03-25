/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/25 13:49
*/
pub mod hashset;

pub trait Solution {
    fn is_path_crossing(path: String) -> bool;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            ("E", false),
            ("NES", false),
            ("NESWW", true),
        ];

        for (path, expected) in test_cases {
            assert_eq!(S::is_path_crossing(path.to_string()), expected);
        }
    }
}