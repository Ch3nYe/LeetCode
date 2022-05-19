/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/5/17 22:40
*/
pub mod bfs;
pub mod dynamic;

pub trait Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[0,0,0],[0,1,0],[0,0,0]] as &[_], &[[0,0,0],[0,1,0],[0,0,0]] as &[_]),
            (&[[0,0,0],[0,1,0],[1,1,1]], &[[0,0,0],[0,1,0],[1,2,1]]),
            // (&[[1,0,1,1,0,0,1,0,0,1],[0,1,1,0,1,0,1,0,1,1],[0,0,1,0,1,0,0,1,0,0],[1,0,1,0,1,1,1,1,1,1],[0,1,0,1,1,0,0,0,0,1],[0,0,1,0,1,1,1,0,1,0],[0,1,0,1,0,1,0,0,1,1],[1,0,0,0,1,1,1,1,0,1],[1,1,1,1,1,1,1,0,1,0],[1,1,1,1,0,1,0,0,1,1]],
            //  &[[1,0,1,1,0,0,1,0,0,1],[0,1,1,0,1,0,1,0,1,1],[0,0,1,0,1,0,0,1,0,0],[1,0,1,0,1,1,1,1,1,1],[0,1,0,1,1,0,0,0,0,1],[0,0,1,0,1,1,1,0,1,0],[0,1,0,1,0,1,0,0,1,1],[1,0,0,0,1,2,1,1,0,1],[2,1,1,1,1,2,1,0,1,0],[3,2,2,1,0,1,0,0,1,1]]),
        ];

        for (mat, expected) in test_cases {
            assert_eq!(S::update_matrix(mat.iter().copied().map(Vec::from).collect()), expected);
        }
    }
}