pub struct Solution;

impl Solution {
    pub fn solve(nums: i32) -> i32 {
        Self::recu(nums,nums)
    }

    pub fn recu(n: i32, m: i32) -> i32 {
        if n==1||m==1 {
            1
        } else if n==m {
            Self::recu(n, m-1)+1
        } else if n<m {
            Self::recu(n,n)
        } else { // n>m
            Self::recu(n-m, m)+Self::recu(n,m-1)
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve(nums: i32) -> i32 {
        Self::solve(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}