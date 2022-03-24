//problem from https://leetcode.com/discuss/interview-question/414064/google-number-of-ways-to-calculate-a-target-number
// solve 1,2 ref: https://developer.aliyun.com/article/445250
pub mod recursive;// solve 1
pub mod dynamic;// solve 2

pub trait Solution {
    fn solve(num: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (4, 5), // 4, 3+1, 2+2, 2+1+1, 1+1+1+1
            (5, 7),
            (6, 11),
        ];

        for (num, expected) in test_cases {
            assert_eq!(S::solve(num), expected);
        }
    }
}