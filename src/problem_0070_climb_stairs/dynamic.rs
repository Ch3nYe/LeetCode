pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        0
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