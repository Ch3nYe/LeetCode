pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        return s.len()==goal.len() && s.repeat(2).contains(&goal);
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn rotate_string(s: String, goal: String) -> bool {
        Self::rotate_string(s, goal)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}