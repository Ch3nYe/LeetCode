pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        if text1.is_empty() || text2.is_empty() {
            return 0;
        }
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let mut dp = vec![vec![0;1000];1000];
        for i in 1..text1.len()+1 {
            for j in 1..text2.len()+1 {
                match text1[i - 1] == text2[j - 1] {
                    true => dp[i][j] = dp[i - 1][j - 1] + 1,
                    _ => dp[i][j] = dp[i - 1][j].max(dp[i][j - 1])
                }
            }
        }
        dp[text1.len()][text2.len()]
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Self::longest_common_subsequence(text1,text2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}