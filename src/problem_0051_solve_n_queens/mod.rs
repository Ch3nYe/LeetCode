/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/20 15:12
*/
pub mod backtract;

pub trait Solution {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [(
            4,
            &[
                &[".Q..", "...Q", "Q...", "..Q."] as &[_],
                &["..Q.", "Q...", "...Q", ".Q.."],
            ] as &[_],
        )];

        for (n, expected) in test_cases {
            assert_eq!(S::solve_n_queens(n), expected);
        }
    }
}