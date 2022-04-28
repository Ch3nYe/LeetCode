pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut seq = vec![0i32;1<<n];
        for i in 0..seq.len() {
            seq[i] = (i >> 1 ^ i) as i32;
        }
        seq
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn gray_code(n: i32) -> Vec<i32> {
        Self::gray_code(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}