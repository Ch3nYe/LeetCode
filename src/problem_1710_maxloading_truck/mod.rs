/*
@author: Ch3nYe
@license: GPL
@contact: sud0su@qq.com
@date: 2022/6/4 10:14
*/
pub mod greedy;

pub trait Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32;
}

#[cfg(test)]
mod tests {
    use super::Solution;

    pub fn run<S: Solution>() {
        let test_cases = [
            (&[[1,3],[2,2],[3,1]] as &[_], 4, 8),
            (&[[5,10],[2,5],[4,7],[3,9]], 10, 91),
        ];

        for (box_types, truck_size, expected) in test_cases {
            assert_eq!(
                S::maximum_units(box_types.iter().copied().map(Vec::from).collect(), truck_size),
                expected
            );
        }
    }
}

