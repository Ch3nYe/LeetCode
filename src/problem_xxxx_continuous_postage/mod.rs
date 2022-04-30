/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/29 21:00
*/
pub mod backtract;

pub trait Solution {
    fn solve_continuous_postage(n: i32, m: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (5, 5, &[1,4,9,31,51] as &[_]),
            (5, 6, &[1,7,12,43,52]),
            (7, 6, &[1,4,18,31,104,145,170]),
        ];

        for (n, m, expected) in test_cases {
            assert_eq!(S::solve_continuous_postage(n,m), expected);
        }
    }
}