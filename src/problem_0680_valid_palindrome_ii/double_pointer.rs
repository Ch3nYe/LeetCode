use std::iter::zip;

pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut fors: Vec<char> = s.chars().collect();
        let mut revs: Vec<char> = s.chars().rev().collect();
        if fors==revs {
            return true;
        }
        for i in 0..s.len() {
            if fors[i] == revs[i]{
                continue;
            }
            let mut tmp = revs[..s.len()-i-1].to_vec();
            let mut tmp2 = fors[i..s.len()-i-1].to_vec();
            tmp2.append(&mut fors[s.len() - i..].to_vec());
            if fors[i+1..] == tmp {
                return true;
            } else if revs[i+1..] == tmp2 {
                return true;
            } else {
                return false
            }
        }
        true
    }
}
// acbefeca
// acefebca
// acefebca fors
// acbefeca rev
// abca fors
// acba revs
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