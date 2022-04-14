pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut b = vec![1;nums.len()];
        for i in 1..nums.len() {
            let mut t = 0;
            for j in 0..i {
                if nums[j] < nums[i] && t<b[j] { t = b[j]; }
                b[i] = t+1;
            }
        }

        *b.iter().max().unwrap()
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn length_of_lis(nums: Vec<i32>) -> i32 {
        Self::length_of_lis(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}