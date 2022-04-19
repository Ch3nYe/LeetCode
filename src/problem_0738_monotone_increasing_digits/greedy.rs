pub struct Solution;
// ref: https://leetcode-cn.com/problems/monotone-increasing-digits/solution/1111lei-jia-fa-by-wincss-zt83/
impl Solution {
    pub fn monotone_increasing_digits(n: i32) -> i32 {
        let mut ones = 111111111; // 9
        let mut res = 0;
        (0..9).for_each(|_| { // every time, find highest digit in n
            while res+ones > n {
                ones /= 10;
            }
            res += ones;
        });
        res

    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn monotone_increasing_digits(n: i32) -> i32 {
        Self::monotone_increasing_digits(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}