use std::collections::BinaryHeap;
use std::cmp::Reverse; // change to min-heap by Reverse()

pub struct Solution;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        let mut ans = Vec::new();
        let n1 = nums1.len();
        let n2 = nums2.len();

        heap.push((Reverse(nums1[0]+nums2[0]), vec![nums1[0], nums2[0]], (0, 0)));
        while let Some((_, pair, (i,j))) = heap.pop() {
            if ans.len() >= k as usize { break; }
            ans.push(pair);
            if i==0 && j+1<n2 {
                heap.push((Reverse(nums1[0]+nums2[j+1]),vec![nums1[0],nums2[j+1]],(0,j+1)));
            }
            if i+1 < n1 {
                heap.push((Reverse(nums1[i+1]+nums2[j]),vec![nums1[i+1],nums2[j]],(i+1,j)));
            }
        }
        ans
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        Self::k_smallest_pairs(nums1, nums2, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}