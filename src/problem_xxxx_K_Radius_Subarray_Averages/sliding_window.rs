pub struct Solution;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, mut k: i32) -> Vec<i32> {
        let mut res = vec![-1 as i32; nums.len()];
        if 2*k >= nums.len() as i32 {
            return res;
        }
        let mut temp:i64 = 0; // must be i64 Otherwise, overflow
        for i in 0..=(2*k) as usize {
            temp += nums[i] as i64;
        }
        res[k as usize] = (temp/(2*k+1) as i64) as i32;
        for i in (k+1) as usize..(nums.len()-k as usize) {
            temp = temp - nums[i-k as usize -1] as i64 + nums[i+k as usize] as i64;
            res[i] = (temp/(2*k+1) as i64) as i32;
        }
        res
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::get_averages(nums,k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}