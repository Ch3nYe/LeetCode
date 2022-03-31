pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MIN;nums.len()];
        dp[0] = nums[0];
        let mut sum = nums[0];
        for i in 1..nums.len() {
            if dp[i-1] > 0 {
                dp[i] = dp[i-1]+nums[i];
            } else {
                dp[i] = nums[i];
            }
            // update max sum
            if dp[i] > sum {
                sum = dp[i];
            }
        }
        sum
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::max_sub_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}