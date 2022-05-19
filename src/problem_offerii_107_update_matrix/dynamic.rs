use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m:usize = mat.len();
        let n:usize = mat[0].len();

        for i in 0..m {
            for j in 0..n {
                if mat[i][j] != 0 {
                    mat[i][j] = i32::MAX/2; // prevent overflow
                }
            }
        }


        for i in 0..m {
            for j in 0..n {
                if i as i32 -1 >= 0 {
                    mat[i][j] = min(mat[i][j], mat[i-1][j] + 1);
                }
                if j as i32 -1 >= 0  {
                    mat[i][j] = min(mat[i][j], mat[i][j-1] + 1);
                }
            }
        }

        for i in (0..m).rev() {
            for j in 0..n {
                if i + 1 < m {
                    mat[i][j] = min(mat[i][j], mat[i+1][j] + 1);
                }
                if j as i32 -1 >= 0  {
                    mat[i][j] = min(mat[i][j], mat[i][j-1] + 1);
                }
            }
        }

        for i in 0..m {
            for j in (0..n).rev() {
                if i as i32 -1 >= 0 {
                    mat[i][j] = min(mat[i][j], mat[i-1][j] + 1);
                }
                if j + 1 < n  {
                    mat[i][j] = min(mat[i][j], mat[i][j+1] + 1);
                }
            }
        }

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i + 1 < m {
                    mat[i][j] = min(mat[i][j], mat[i+1][j] + 1);
                }
                if j + 1 < n  {
                    mat[i][j] = min(mat[i][j], mat[i][j+1] + 1);
                }
            }
        }

        mat
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Self::update_matrix(mat)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}