pub struct Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut res = Vec::new();
        for i in 0..12{ // hours
            for j in 0..60{ // minutes
                if (i as i32).count_ones() + (j as i32).count_ones() == turned_on as u32 {
                    let mut s = String::new();
                    s.push_str(&(i.to_string()));
                    s.push_str(":");
                    if j < 10 { s.push_str("0"); }
                    s.push_str(&(j.to_string()));
                    res.push(s);
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