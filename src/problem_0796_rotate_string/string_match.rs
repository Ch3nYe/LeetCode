pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        let n = s.len();
        let s:Vec<char> = s.chars().collect();
        let goal:Vec<char> = goal.chars().collect();
        (0..n).any(|i| (0..n).all(|j| s[(i+j)%n]==goal[j]))
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