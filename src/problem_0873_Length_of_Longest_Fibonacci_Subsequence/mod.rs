/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/16 12:59
*/
pub mod dp_hash;
pub mod dp_binary_search;

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
            (&[1,3,5], 0),
        ];

        for (nums, expected) in test_cases {
            assert_eq!(S::len_longest_fib_subseq(nums.to_vec()), expected);
        }
    }
}