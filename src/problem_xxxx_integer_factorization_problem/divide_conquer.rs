
pub struct Solution;

impl Solution {
    pub fn interget_factorization(num: i32) -> i32 {
        let mut count = 0;
        if num == 1 {
            return 1;
        }
        for i in 2..=num {
            if num%i == 0 {
                count += Self::interget_factorization(num/i);
            }
        }
        count
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