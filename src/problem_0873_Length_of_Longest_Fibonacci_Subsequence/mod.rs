/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/16 12:59
*/
pub mod xxx;

pub trait Solution {
    fn len_longest_fib_subseq(arr: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[1,2,3,4,5,6,7,8] as &[_], 5),
            (&[1,3,7,11,12,14,18], 3),

        ];

        for (nums, target, expected) in test_cases {
            assert_eq!(S::num_subseq(nums.to_vec(), target), expected);
        }
    }
}