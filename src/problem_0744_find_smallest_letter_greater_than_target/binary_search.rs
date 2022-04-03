use std::thread::sleep;
pub struct Solution;

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let t = (target as u8 + 1) as char;
        let idx = match letters.binary_search(&t) {
            // when binary_search() not found the element,
            // Result::Err is returned, containing the index
            // where a matching element could be inserted while maintaining sorted order.
            Ok(i) => i,
            Err(i) => i,
        } % letters.len();
        letters[idx]
    }
}
// https://leetcode-cn.com/problems/find-smallest-letter-greater-than-target/solution/744-xun-zhao-bi-mu-biao-zi-mu-da-de-zui-t34jn/


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        Self::next_greatest_letter(letters,target)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}