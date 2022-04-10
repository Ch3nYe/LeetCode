pub struct Solution;

// dp[1] = 1, dp[2] = 2
// dp[n] = dp[n-1] + dp[n-2]
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a,mut b,mut c) = (0,1,1);
        for i in 1..=n {
            c = a + b;
            a = b;
            b = c;
        }
        c
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn climb_stairs(n: i32) -> i32 {
        Self::climb_stairs(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}