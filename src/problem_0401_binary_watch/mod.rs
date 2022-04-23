/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/22 23:19
*/

pub mod brute_force;
// https://leetcode-cn.com/problems/binary-watch/
pub trait Solution {
    fn read_binary_watch(turned_on: i32) -> Vec<String>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"] as &[_]),
        ];

        for (turned_on, expected) in test_cases {
            assert_eq!(S::read_binary_watch(turned_on), expected);
        }
    }
}
