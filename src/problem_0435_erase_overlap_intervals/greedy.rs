pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        intervals.sort_unstable_by_key(|x| x[1]);

        let mut remove_count = 0;
        let mut j :usize = 0;
        for i in 1..intervals.len() {
            if intervals[i][0] >= intervals[j][1] {
                j=i;
            } else {
                remove_count += 1;
            }
        }
        remove_count
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