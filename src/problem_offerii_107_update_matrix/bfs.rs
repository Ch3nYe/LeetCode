use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn update_matrix(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue:VecDeque<Vec<usize>> = VecDeque::new();
        let m:usize = mat.len();
        let n:usize = mat[0].len();
        let mut checked:Vec<Vec<bool>> = vec![vec![false; n];m];

        // init checked matrix, and push all 0 point to queue.
        for i in 0..m {
            for j in 0..n {
                if mat[i][j] == 0 {
                    queue.push_back(vec![i, j]);
                    checked[i][j] = true;
                }
            }
        }

        // bfs
        let neighbors:Vec<Vec<i32>> = vec![vec![-1,0],vec![1,0],vec![0,-1],vec![0,1]];
        while !queue.is_empty() {
            if let Some(loc) = queue.pop_front(){
                for index in 0..4 {
                    let new_x:usize = loc[0].wrapping_add(neighbors[index][0] as usize);
                    let new_y:usize = loc[1].wrapping_add(neighbors[index][1] as usize);
                    if new_x < m && new_y < n && !checked[new_x][new_y] {
                        mat[new_x][new_y] = mat[loc[0]][loc[1]]+1;
                        checked[new_x][new_y] = true;
                        queue.push_back(vec![new_x, new_y]);
                    }
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