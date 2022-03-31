/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/30 21:26
*/

pub mod dynamic;

pub trait Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[-2,1,-3,4,-1,2,1,-5,4] as &[_], 6),
            (&[1], 1),
            (&[5,4,-1,7,8], 23),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::max_sub_array(nums.to_vec()), expected);
        }
    }
}