use std::ops::Deref;

pub struct Solution;

impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        patterns.iter().fold(0,|res, s| {if word.contains(&*s) {res+1} else {res}})
        // patterns.iter().fold(0,|mut res, s| {if word.contains(s.deref()) {res+=1}; res})

    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        Self::num_of_strings(patterns, word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}