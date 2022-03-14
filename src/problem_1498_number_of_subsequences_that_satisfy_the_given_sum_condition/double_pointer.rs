pub struct Solution;

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        nums.sort();
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i] + nums[j] <= target {
                    res += 1;
                    if i==j {
                        res+=1;
                    }
                } else {
                    break
                }
            }
        }
        res % 1000000007
    }
}