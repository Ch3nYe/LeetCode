pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let len = s.len();
        let sv: Vec<char> = s.chars().collect();
        let mut j = len;
        for i in 0..len {
            j -= 1;
            if sv[i] != sv[j] {
                return Self::is_palindrome(&s,i+1,j) || Solution::is_palindrome(&s,i,j-1);
            }
        }
        true
    }
    pub fn is_palindrome(s:&str, mut i:usize, mut j:usize) -> bool {
        let sv: Vec<char> = s.chars().collect();
        while i < j {
            if sv[i] != sv[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_palindrome(s: String) -> bool {
        Self::valid_palindrome(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}