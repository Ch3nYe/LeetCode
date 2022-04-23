pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        ["1"]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn read_binary_watch(turned_on: i32) -> Vec<String> {
        Self::read_binary_watch(turned_on)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}