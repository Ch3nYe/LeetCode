/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/6 13:39
*/
pub mod dynamic;

// https://leetcode-cn.com/problems/minimum-score-triangulation-of-polygon/
pub trait Solution {
    fn min_score_triangulation(values: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[3,7,4,5] as &[_], 144),
            (&[1,2,3] as &[_], 6),
            (&[1,3,1,4,1,5], 13),
        ];

        for (values, expected) in test_cases {
            assert_eq!(S::min_score_triangulation(values.to_vec()), expected);
        }
    }
}