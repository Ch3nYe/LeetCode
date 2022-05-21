/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/5/21 18:49
*/
pub mod iterative;
// ref: https://github.com/EFanZh/LeetCode/blob/master/src/problem_0782_transform_to_chessboard/iterative.rs

pub trait Solution {
    fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::test_utilities::Matrix;

    pub fn run<S: Solution>() {
        let test_cases = [
            (
                &[[0, 1, 1, 0], [0, 1, 1, 0], [1, 0, 0, 1], [1, 0, 0, 1]] as &dyn Matrix<_>,
                2,
            ),
            (&[[0, 1], [1, 0]], 0),
            (&[[1, 0], [1, 0]], -1),
            (&[[1, 1, 0], [0, 0, 1], [0, 0, 1]], 2),
            (&[[1, 0, 0], [0, 1, 1], [1, 0, 0]], 1),
            (&[[0, 0, 1, 1], [1, 1, 0, 0], [0, 1, 0, 1], [1, 0, 1, 0]], -1),
            (&[[1, 1, 1, 1], [1, 1, 1, 1], [0, 0, 0, 0], [0, 0, 0, 0]], -1),
        ];

        for (board, expected) in test_cases {
            assert_eq!(S::moves_to_chessboard(board.to_vec()), expected);
        }
    }
}