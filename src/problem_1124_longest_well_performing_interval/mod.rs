/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/3/26 22:51
*/

pub mod dynamic; //
pub mod presum_monotonous_stack; // https://leetcode-cn.com/problems/longest-well-performing-interval/solution/qian-zhui-he-dan-diao-zhan-python3-by-smoon1989/

pub trait Solution {
    fn longest_wpi(hours: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[7,8,7,9] as &[_], 1),
            (&[9,9,6,0,6,6,9] as &[_], 3),
            (&[6,6,6], 0),
        ];

        for (hours, expected) in test_cases {
            assert_eq!(S::longest_wpi(hours.to_vec()), expected);
        }
    }
}