pub struct Solution;

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let mut result: usize = 0;
        let n = n as usize;

        Self::solve_n_queens_helper(
            n,
            0,
            &mut result,
            &mut vec![true; n],
            &mut vec![true; 2*n-1],
            &mut vec![true; 2*n-1],
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
        hit_column: &mut [bool],
        hit_main_diago: &mut [bool],
        hit_back_diago: &mut [bool],

    ) {
        if row>=n {
            *count += 1;
        } else {
            for column in 0..n {
                let main_diago = column+n-row-1;
                let back_diago = column+row;

                if hit_column[column] && hit_main_diago[main_diago] && hit_back_diago[back_diago] {
                    hit_column[column] = false;
                    hit_main_diago[main_diago] = false;
                    hit_back_diago[back_diago] = false;

                    Self::solve_n_queens_helper(n, row+1, count, hit_column, hit_main_diago, hit_back_diago);

                    hit_column[column] = true;
                    hit_main_diago[main_diago] = true;
                    hit_back_diago[back_diago] = true;
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