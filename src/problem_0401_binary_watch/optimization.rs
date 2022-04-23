pub struct Solution;

// ref: https://leetcode-cn.com/problems/binary-watch/solution/yi-kong-jian-huan-shi-jian-bi-mian-zhong-bkrq/

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        if turned_on < 0 || turned_on > 8 {
            return vec![];
        }

        let mut res = Vec::new();

        // record hours correspond to light
        let mut hours_turn_on = vec![vec![]; 4]; // 存储i个灯亮时可能的小时数字
        for hour in (0i32..12i32) {
            hours_turn_on[hour.count_ones() as usize].push(format!("{}", hour));
        }
        // record minutes correspond to light
        let mut minutes_turn_on: Vec<Vec<String>> = vec![vec![]; 6]; // 存储i个灯亮时可能的分钟数字
        for minute in (0i32..60i32) {
            minutes_turn_on[minute.count_ones() as usize].push(format!("{:02}", minute));
        }

        // 通过上面的空间 将之前的12x60的搜索次数 优化为4
        for hour in (0i32..4i32) {
            let mut minute = turned_on - hour;
            if minute >=0 && minute <= 5 {
                for h in &hours_turn_on[hour as usize] {
                    for m in &minutes_turn_on[minute as usize]{
                        res.push(format!("{}:{}",h,m));
                    }
                }
            }
        }
        res
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn read_binary_watch(turned_on: i32) -> Vec<String> {
        Self::read_binary_watch(turned_on)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}