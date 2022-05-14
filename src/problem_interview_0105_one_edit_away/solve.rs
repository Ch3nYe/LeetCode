pub struct Solution;

impl Solution {
    pub fn one_edit_away(first: String, second: String) -> bool {
        let (m,n) = (first.len(),second.len());
        if m<n {
            return Self::one_edit_away(second.clone(),first.clone());
        }
        if m-n>1 {
            return false;
        }

        // first.len() == second.len()
        let (first,second) = (first.as_bytes(),second.as_bytes());
        for i in 0..n {
            if first[i]!=second[i] {
                return if m==n {first[i+1..] == second[i+1..]} else {first[i+1..] == second[i..]}
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn one_edit_away(first: String, second: String) -> bool {
        Self::one_edit_away(first,second)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}