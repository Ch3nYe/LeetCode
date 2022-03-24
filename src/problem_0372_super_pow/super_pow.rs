pub struct Solution;

impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        let res = 1;
        const c: i32 = 1337;
        const phi_c: i32 = 1140;
        let n = b.len();
        if n == 0 {
            return res;
        }
        // (a^b)%c == (a^(b%phi(c))) % c
        // where c is a prime number and phi(c) is an euler function: https://zh.wikipedia.org/wiki/%E6%AC%A7%E6%8B%89%E5%87%BD%E6%95%B0

        // calc b%phi(c)
        let mut exp = 0;
        for i in b {
            exp = (10*exp+i)%phi_c;
        }
        Self::quick_pow(a as i64,exp as i64,c as i64) as i32
    }

    fn quick_pow(mut a: i64, mut b: i64, c: i64) -> i64 {
        let mut res = 1;
        while b!=0 {
            if b%2 == 1 {
                res = res*a%c;
            }
            a = a*a%c;
            b = b/2;
        }
        res
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        Self::super_pow(a, b)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}