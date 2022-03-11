pub mod recursive;// 解法1

pub trait Solution {
    fn solve(nums: i32) -> i32;
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

        for (nums, expected) in test_cases {
            assert_eq!(S::solve(nums), expected);
        }
    }
}