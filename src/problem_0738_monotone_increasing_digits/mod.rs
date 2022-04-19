/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/19 21:54
*/
pub mod greedy;

pub trait Solution {
    fn monotone_increasing_digits(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (10,9),
            (1234,1234),
            (332,299),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::monotone_increasing_digits(n), expected);
        }
    }
}