/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/4/12 10:02
*/
pub mod dynamic;
// https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/
pub trait Solution {
    fn max_profit(prices: Vec<i32>) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[7,1,5,3,6,4] as &[_], 5),
            (&[7,6,4,3,1], 0),
        ];

        for (prices, expected) in test_cases {
            assert_eq!(S::max_profit(prices.to_vec()), expected);
        }
    }
}