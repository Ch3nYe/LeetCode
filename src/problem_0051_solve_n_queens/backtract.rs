pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let n = n as usize;

        Self::solve_n_queens_helper(
            n,
            0,
            &mut vec![b'.'; n*n],
            &mut vec![true; n],
            &mut vec![true; 2*n-1],
            &mut vec![true; 2*n-1],
            &mut result
        );

        result
    }
    ///
    /// recursively process every row
    ///
    pub fn solve_n_queens_helper(
        n: usize, // size of board
        row: usize, // current row index
        base: &mut [u8], // current board in n*n char vector
        columns_hits: &mut [bool], // columns set recording Whether or not could put a queen in column idx in current row
        main_diagonal_hits: &mut [bool],
        back_diagonal_hits: &mut [bool],
        result: &mut Vec<Vec<String>>
    ) {
        if row == n { // find a solution, add base to result
            result.push(
                base.chunks_exact(n) // return a iter, which will extract the first n element every time
                    .map(|row| String::from_utf8(row.to_vec()).unwrap())
                    .collect(),
            );
        } else {
            for column in 0..n {
                let main_diagonal = column+n-row-1; // attention, usize overflow panic
                let back_diagonal = row+column;

                if columns_hits[column] && main_diagonal_hits[main_diagonal] && back_diagonal_hits[back_diagonal] { // if this position could put a queen
                    base[n*row+column] = b'Q';
                    columns_hits[column] = false;
                    main_diagonal_hits[main_diagonal] = false;
                    back_diagonal_hits[back_diagonal] = false;

                    // deep first search
                    Self::solve_n_queens_helper(
                        n,
                        row+1,
                        base,
                        columns_hits,
                        main_diagonal_hits,
                        back_diagonal_hits,
                        result,
                    );

                    // backtract
                    base[n*row+column] = b'.';
                    columns_hits[column] = true;
                    main_diagonal_hits[main_diagonal] = true;
                    back_diagonal_hits[back_diagonal] = true;
                }
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        Self::solve_n_queens(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}