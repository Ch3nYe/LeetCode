pub struct Solution;

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut seq = Vec::new();
        seq.push(0);
        for i in 1..=n{
            let m = seq.len();
            for j in (0..m).rev() {
                seq.push(seq[j]|(1<<(i-1)));
            }
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