pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut result: usize = 0;
        let n = n as usize;

        Self::solve_n_queens_helper(
            n,
            0,
            &mut result,
            !0,
            !0,
            !0,
        );

        result as i32
    }
    ///
    /// recursively process every row
    ///
    pub fn solve_n_queens_helper(
        n: usize,
        row: usize,
        count: &mut usize,
        mut hit_column: u32,
        mut hit_main_diago: u32,
        mut hit_back_diago: u32,

    ) {
        if row>=n {
            *count += 1;
        } else {
            for column in 0..n {
                let main_diago = column+n-row-1;
                let back_diago = column+row;
                let av = (hit_column>>column&1)&(hit_main_diago>>main_diago&1)&(hit_back_diago>>back_diago&1);
                if av!=0 {
                    hit_column ^= 1<<column;
                    hit_main_diago ^= 1<<main_diago;
                    hit_back_diago ^= 1<<back_diago;

                    Self::solve_n_queens_helper(n, row+1, count, hit_column, hit_main_diago, hit_back_diago);

                    hit_column ^= 1<<column;
                    hit_main_diago ^= 1<<main_diago;
                    hit_back_diago ^= 1<<back_diago;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn total_n_queens(n: i32) -> i32 {
        Self::total_n_queens(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}