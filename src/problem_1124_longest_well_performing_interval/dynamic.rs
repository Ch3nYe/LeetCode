use std::cmp::{max, min};

pub struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut res:i32 = 0;
        let mut diff:i32 = 0;
        let mut dp: Vec<i32> = vec![-1; 10001];
        for i in 0..hours.len() {
            diff += if hours[i]>8 {1} else {-1};
            if diff>0 {
                res = max(res, (i + 1) as i32);
            } else {
                if dp[(diff + 10000) as usize] != -1 {
                    res = max(res, i as i32 - dp[(diff + 10000) as usize] );
                }
                if diff < 0 {
                    if -1 == dp[(diff + 10001) as usize] {dp[(diff + 10001) as usize] = i as i32;}
                    if -1 != dp[(diff + 10000) as usize] {dp[(diff + 10001) as usize] = min(dp[(diff + 10001) as usize],dp[(diff + 10000) as usize]);}
                }
            }
        }
        res
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_wpi(hours: Vec<i32>) -> i32 {
        Self::longest_wpi(hours)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}