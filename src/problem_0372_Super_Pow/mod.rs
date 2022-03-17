/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/17 15:54
*/
pub mod super_pow;

pub trait Solution {
    fn super_pow(a: i32, b: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (1, &[4,3,3,8,5,2] as &[_], 1),
            (2, &[3], 8),
            (2, &[1,0], 1024),
            (2147483647, &[2,0,0], 1198),

        ];

        for (a, b, expected) in test_cases {
            assert_eq!(S::super_pow(a, b.to_vec()), expected);
        }
    }
}