pub struct Solution;


impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_inclusion(s1: String, s2: String) -> bool {
        Self::check_inclusion(s1,s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}