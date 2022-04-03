/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/3 21:18
*/
pub mod binary_search;

pub trait Solution {
    fn next_greatest_letter(letters: Vec<char>, target: char) -> char;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&['c', 'f', 'j'] as &[_], 'a', 'c'),
            (&['c', 'f', 'j'], 'c', 'f'),
            (&['c', 'f', 'j'], 'd', 'f'),
            (&['a', 'b', 'c', 'f'], 'd', 'f'),
        ];

        for (letters, target, expected) in test_cases {
            assert_eq!(S::next_greatest_letter(letters.to_vec(), target), expected);
        }
    }
}