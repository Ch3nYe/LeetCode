pub struct Solution;

impl Solution {
    pub fn num_subseq(nums0: Vec<i32>, target: i32) -> i32 {
        let mut nums = Vec::from(nums0);
        let m = 1e9 as i32+7;
        let mut res:i32 = 0;
        let mut left:usize = 0;
        let mut right:usize = nums.len()-1;
        nums.sort();
        if nums[0] * 2 > target {
            return 0;
        }
        while left<=right {
                if nums[left] + nums[right] <= target {
                    res = (res+Self::qpow(2,right - left)) % m; // diff from res += x % 1000000007
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        res % m
    }
    fn qpow(mut n:i64, mut p:usize) -> i32 { // 快速幂算法
        let m = 1e9 as i64+7;
        let mut res:i64 = 1;
        while(p != 0){
            if(p%2 == 1) {
                res = (res * n) % m
            };
            n = (n * n) % m;
            p >>= 1;
        }
        (res % m) as i32
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        Self::num_subseq(nums, target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}