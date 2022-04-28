/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/28 11:02
*/

pub mod binary2gray;
pub mod symmetry;
pub mod backtrack;

pub trait Solution {
    fn gray_code(n: i32) -> Vec<i32>;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &[0,1] as &[_]),
            (2, &[0,1,3,2]),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::gray_code(n), expected);
        }
    }
}