pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut k = 1;
        let mut b = vec![0;nums.len()+1]; // b[k] is the min tail number in sequence that length of mono sequence == k
        b[1] = nums[0];
        for i in 1..nums.len() {
            if nums[i] > b[k] { k+=1; b[k]=nums[i];} // add a number to mono sequence
            else { // update b vec
                if nums[i] < b[1] { b[1] = nums[i]; } // mini number update to b[1]
                else { // find a appropriate position in b vec
                    let idx = match b[..(k+1)].binary_search(&nums[i]) {
                        Ok(idx) => idx-1,
                        Err(idx) => idx,
                    };
                    if b[idx]>nums[i] { // only the b[idx] < nums[i], we should update b[idx]
                        b[idx]=nums[i];
                    }
                }
            }
        }
        k as i32
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