pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut k = 1;
        let mut b = vec![0;nums.len()];
        b[1] = nums[0];
        for i in 1..nums.len() {
            if nums[i]>=b[k] { k+=1; b[k]=nums[i];}
            else {
                if nums[i] < b[1] { b[1] = nums[i]; }
                else {
                    let idx = match b.binary_search(&nums[i]) {
                        Ok(idx) => idx-1,
                        Err(idx) => idx,
                    };
                    b[idx]=nums[i];
                }
                // let idx = Self::bisearch(nums.to_vec(), b.to_vec(), i, k);
                // b[idx]=nums[i];
            }
        }
        k as i32
    }
    fn bisearch(nums: Vec<i32>, b: Vec<i32>, i: usize, mut k: usize) -> usize {
        if nums[i] < b[1] { return 1; }
        let mut h=1;
        let mut j=k;
        while h!=(j-1) {
            k = (h+j)/2;
            if b[k]<=nums[i] {
                h = k;
            } else { j=k; }
        }
        return j;
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