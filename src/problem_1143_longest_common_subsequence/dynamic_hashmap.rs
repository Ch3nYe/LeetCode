use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        if text1.is_empty() || text2.is_empty() {
            return 0;
        }
        let (text1, text2) = (text1.as_bytes(), text2.as_bytes());
        let mut dp:HashMap<String, i32> = HashMap::new();
        for i in 1..text1.len()+1 {
            for j in 1..text2.len()+1 {
                dp.entry(format!("{}-{}",i-1,j-1)).or_insert(0);
                dp.entry(format!("{}-{}",i,j-1)).or_insert(0);
                dp.entry(format!("{}-{}",i-1,j)).or_insert(0);
                match text1[i - 1] == text2[j - 1] {
                    true => dp.insert(format!("{}-{}",i,j), dp.get(&*format!("{}-{}", i - 1, j - 1)).unwrap()+1),
                    _ => dp.insert(format!("{}-{}",i,j), *dp.get(&*format!("{}-{}", i - 1, j)).unwrap().max(dp.get(&*format!("{}-{}", i, j - 1)).unwrap()))
                };
            }
        }
        *dp.get(&*format!("{}-{}", text1.len(), text2.len())).unwrap()
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        Self::longest_common_subsequence(text1,text2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}