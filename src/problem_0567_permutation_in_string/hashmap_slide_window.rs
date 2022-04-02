pub struct Solution;


impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len()>s2.len() {
            return false;
        }

        let mut s1_count = vec![0;26];
        s1.chars().for_each( |c| {s1_count[c as usize - 97] += 1});
        let s2:Vec<char> = s2.chars().collect(); // To facilitate index
        let mut s2_count = vec![0;26];
        s2[..s1.len()].iter().for_each(|&c| {s2_count[c as usize - 97] += 1});

        if s2_count == s1_count { return true; }
        for right in (s1.len()-1)..(s2.len()-1) {
            // slide window update s2_count
            s2_count[s2[right-(s1.len()-1)] as usize - 97] -= 1;
            s2_count[s2[right+1] as usize - 97] += 1;
            // judge
            if s2_count == s1_count {
                return true;
            }
        }
        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_inclusion(s1: String, s2: String) -> bool {
        Self::check_inclusion(s1,s2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}