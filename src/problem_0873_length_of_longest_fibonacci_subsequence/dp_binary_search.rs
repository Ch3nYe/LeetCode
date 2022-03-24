pub struct Solution;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![0; arr.len()]; arr.len()];

        for i in 1..arr.len() {
            for j in 0..i {
                let temp = arr[i]-arr[j];
                let k = Self::binary_search(&arr,j as i32,temp);
                if k>-1 && k < j as i32 {
                    dp[i][j] = dp[j][k as usize] + 1;
                } else {
                    dp[i][j] = 2;
                }
                res = res.max(dp[i][j]);
            }
        }
        if res>2 { res } else { 0 }
    }

    fn binary_search(data: &Vec<i32>, mut r: i32, target: i32) -> i32 {
        let mut l: i32 = 0;
        // 左闭右开区间
        while l < r {
            let mid = l + (r - l) / 2;
            if data[mid as usize] == target {
                return mid;
            }
            if data[mid as usize] < target {
                l = mid + 1;
            } else {
                r = mid
            }
        }
        return -1;
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