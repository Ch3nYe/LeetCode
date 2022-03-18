/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/18 23:32
*/

pub mod divide_conquer;

pub trait Solution {
    fn interget_factorization(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (12, 8),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::interget_factorization(num), expected);
        }
    }
}