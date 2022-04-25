/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/25 19:47
*/
pub mod backtract;
pub mod backtract2;

pub trait Solution {
    fn total_n_queens(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, 1),
            (2, 0),
            (3, 0),
            (4, 2),
            (5, 10),
            (6, 4),
            (7, 40),
            (8, 92),
            (9, 352),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::total_n_queens(n), expected);
        }
    }
}