pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        Self::erase_overlap_intervals(intervals)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}