pub struct Solution;

impl Solution {
    pub fn dynamic(num: i32) -> i32 {
        let max = 122;
        let mut dp = vec![vec![0; max] ; max];
        for i in 1..max {
            dp[i][1] = 1;
            dp[1][i] = 1;
            dp[0][i] = 1;
        }
        for i in 2..max {
            for j in 2..max {
                if j<=i {
                    dp[i][j] = dp[i][j-1] + dp[i-j][j];
                } else {
                    dp[i][j] = dp[i][i];
                }
            }
        }
        dp[num as usize][num as usize]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve(num: i32) -> i32 {
        Self::dynamic(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}