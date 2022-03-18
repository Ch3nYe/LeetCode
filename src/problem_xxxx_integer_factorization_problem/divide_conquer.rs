
pub struct Solution;

impl Solution {
    pub fn interget_factorization(num: i32) -> i32 {
        0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn interget_factorization(num: i32) -> i32 {
        Self::interget_factorization(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}