use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![0; arr.len()]; arr.len()];
        let mut map:HashMap<i32, usize> = HashMap::new();
        for (idx,&num) in arr.iter().enumerate() {
            map.insert(num, idx);
        }
        for i in 1..arr.len() {
            for j in 0..i {
                let temp = arr[i]-arr[j];
                if map.contains_key(&temp) && *map.get(&temp).unwrap()<j {
                    dp[i][j] = dp[j][*map.get(&temp).unwrap()] + 1;
                } else {
                    dp[i][j] = 2;
                }
                res = res.max(dp[i][j]);
            }
        }
        if res>2 { res } else { 0 }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        Self::len_longest_fib_subseq(arr)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}