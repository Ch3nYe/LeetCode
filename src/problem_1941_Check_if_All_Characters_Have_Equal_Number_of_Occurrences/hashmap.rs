use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map = HashMap::new();

        for ch in s.chars() {
            let mut counter = map.entry(ch).or_insert(0);
            *counter += 1;
        }

        let mut counter = 0;
        for (key,value) in &map { // map.iter() 会增加内存消耗
            if counter == 0{
                counter = *value;
            } else {
                if counter!=*value {
                    return false
                }
            }
        }
        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn are_occurrences_equal(s: String) -> bool {
        Self::are_occurrences_equal(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}