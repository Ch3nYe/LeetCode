pub struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let len = values.len();
        let mut dp = vec![vec![0;50];50]; // dp[i][j] means that from i to j, the min_score_triangulation

        for i in (0..=len-3).rev() {
            for j in i+2..len {
                let mut temp_min = dp[i][i+1]+dp[i+1][j]+values[i]*values[i+1]*values[j];
                for k in i+2..j {
                    temp_min = temp_min.min(dp[i][k]+dp[k][j]+values[i]*values[k]*values[j]);
                }
                dp[i][j] = temp_min;
            }
        }
        dp[0][len-1]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_score_triangulation(values: Vec<i32>) -> i32 {
        Self::min_score_triangulation(values)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}