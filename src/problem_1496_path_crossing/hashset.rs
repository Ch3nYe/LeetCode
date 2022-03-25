use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut map= HashSet::new();
        let mut path = path.chars();
        let (mut x, mut y) = (0,0);
        map.insert((x,y));

        while let Some(i) = path.next() {
            match i {
                'N' =>x+=1,
                'E' =>y+=1,
                'S' =>x-=1,
                'W' =>y-=1,
                _ => ()
            };
            if map.contains(&(x, y)) {
                return true;
            }
            map.insert((x, y));
        }
        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_path_crossing(path: String) -> bool {
        Self::is_path_crossing(path)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}