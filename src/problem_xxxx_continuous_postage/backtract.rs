pub struct Solution;

static NUM: usize = 10; // 最大邮票种类数
static LEN: usize = 10000; // 最大postage上限

impl Solution {
    pub fn solve_continuous_postage(n: i32, m: i32) -> Vec<i32> {
        let mut x = vec![0i32; NUM]; // n种邮票的面值
        let mut cnt = 0 as usize; // 当前处理的邮票种类数
        let mut r = vec![0; NUM]; // 结果 解向量
        let knd = n as usize; // 邮票种类
        let lim = m; // 限制张数
        let mut max = 0; // 记录连续postage最大值

        let mut C = vec![vec![0i32;NUM];LEN];

        Self::dfs(knd,cnt,lim,&mut C,&mut x,&mut r, max);

        r
    }
    pub fn findMax(cnt: usize, C: &mut Vec<Vec<i32>>, x: &mut Vec<i32>, lim:i32) -> i32 {
        let mut j = 1;
        while C[cnt - 1][j]>0 {
            if j < x[cnt] as usize || C[cnt - 1][j] <= C[cnt][j - x[cnt] as usize] + 1 {
                C[cnt][j] = C[cnt - 1][j];
            } else {
                C[cnt][j] = C[cnt][j - x[cnt] as usize] + 1;
            }
            j += 1;
        }

        loop {
            let mut tmp = i32::MAX;
            for i in 1..=cnt {
                if tmp > C[cnt][j - x[i] as usize] + 1 {
                    tmp = C[cnt][j - x[i] as usize] + 1;
                }
            }
            if tmp == i32::MAX || tmp > lim {
                break;
            } else {
                C[cnt][j] = tmp;
            }
            j += 1;
        }
        C[cnt][j] = 0;
        return (j - 1) as i32;
    }
    pub fn dfs(
        knd: usize,
        mut cnt: usize,
        lim: i32,
        C: &mut Vec<Vec<i32>>,
        x: &mut Vec<i32>,
        r: &mut Vec<i32>,
        mut max: i32
    ) {
        if cnt == knd {
            if x[cnt] * lim < max {
                return;
            }
            let tmp = Self::findMax(cnt,C,x,lim);
            if tmp > max {
                max = tmp;
                for i in 1..=knd {
                    r[i] = x[i];
                }
            }
        } else {
            let tmp = Self::findMax(cnt,C,x,lim);
            for i in ((tmp + 1)..=(x[cnt as usize] + 1)).rev() {
                cnt += 1;
                x[cnt] = i;
                Self::dfs(knd,cnt,lim,C,x,r,max);
                cnt-=1;
            }
        }
    }
}


// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn solve_continuous_postage(n: i32, m: i32) -> Vec<i32> {
        Self::solve_continuous_postage(n,m)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}