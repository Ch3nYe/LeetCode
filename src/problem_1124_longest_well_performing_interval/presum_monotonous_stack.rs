use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn longest_wpi(hours: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut score:Vec<i32> = Vec::new();
        for i in hours {
            score.push(if i>8 {1} else {-1});
        }
        let mut temp = 0;
        let mut stack_top_p = 0;
        let mut presum:Vec<i32> = Vec::new();
        let mut mono_stack = Vec::new();
        presum.push(temp);
        mono_stack.push(0 as usize);
        for (idx,&i) in score.iter().enumerate() {
            temp+=i;
            presum.push(temp);
            if stack_top_p > temp {
                mono_stack.push(idx+1);
                stack_top_p = temp;
            }
        }

        // 单调栈 求最大上坡长度
        let mut i = presum.len()-1;
        while i > res {
            while mono_stack.len() >= 1 && presum[i]>presum[mono_stack[mono_stack.len()-1]] {
                res = max(res as i32,i as i32 -mono_stack.pop().unwrap() as i32) as usize;
            }
            i -= 1;
        }
        res as i32
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_wpi(hours: Vec<i32>) -> i32 {
        Self::longest_wpi(hours)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}