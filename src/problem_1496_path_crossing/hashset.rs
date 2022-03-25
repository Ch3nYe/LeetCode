pub struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_path_crossing(path: String) -> bool {
        Self::is_path_crossing(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}