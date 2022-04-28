pub struct Solution;

impl Solution {

    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut seq = Vec::new();

        Self::dfs(n, 0, 0, &mut seq, 0);

        seq
    }

    pub fn dfs(n: i32, length: i32, mut s: i32, seq: &mut Vec<i32>, flag: i32) {
        if length == n {
            seq.push(s);
        } else {
            if flag == 0 {
                s = s<<1;
                s = s & (i32::MAX<<1); // set right bit 0
                Self::dfs(n,length+1,s,seq, 0);
                s = s | 1;
                Self::dfs(n,length+1,s,seq, 1);
                s = s>>1;
            } else {
                s = s<<1;
                s = s | 1;
                Self::dfs(n,length+1,s,seq, 0);
                s = s & (i32::MAX<<1); // set right bit 0
                Self::dfs(n,length+1,s,seq, 1);
                s = s>>1;
            }
        }
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