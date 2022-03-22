pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_palindrome(s: String) -> bool {
        Self::valid_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}