pub struct Solution;

impl Solution {
    pub fn maximum_gap(nums: Vec<i32>) -> i32 {
        let len = nums.len() as i32;
        if len < 2 {
            return 0
        }

        let (mut min, mut max) = (
            *nums.iter().min().unwrap(),
            *nums.iter().max().unwrap(),
        );

        let avrGap = ((max - min)/(len - 1)); // 2
        let size = len;
        let mut buckets = vec![None; size as usize];
        nums.into_iter().for_each(|val| {
            let b = &mut buckets[((val - min)/avrGap) as usize];
            if let Some((x, y)) = b {
                *x = val.min(*x);
                *y = val.max(*y);
            } else {
                *b = Some((val, val));
            }
        });

        buckets
            .into_iter()
            .filter_map(|x| x)
            .fold((0, min), |(res, prv), (min, max)| (res.max(min - prv), max))
            .0
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_gap(nums: Vec<i32>) -> i32 {
        Self::maximum_gap(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}