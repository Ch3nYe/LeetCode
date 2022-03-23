pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        let mut fors: Vec<char> = s.chars().collect();
        let mut revs: Vec<char> = s.chars().rev().collect();
        if fors==revs {
            return true;
        }
        for i in 0..(s.len()/2) {
            if fors[i] == revs[i]{
                continue;
            }
            let mut tmp = revs[i..s.len()-i-1].to_vec();
            tmp.append(&mut revs[s.len() - i..].to_vec());
            if fors[i+1..] == tmp {
                return true;
            }
            let mut tmp = fors[i..s.len()-i-1].to_vec();
            tmp.append(&mut fors[s.len() - i..].to_vec());
            if revs[i+1..] == tmp {
                return true;
            }
            return false
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