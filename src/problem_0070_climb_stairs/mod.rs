/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/9 21:04
*/

pub mod dynamic;

pub trait Solution {
    fn climb_stairs(n: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (2, 2),
            (3, 3),
        ];

        for (n, expected) in test_cases {
            assert_eq!(S::climb_stairs(n), expected);
        }
    }
}