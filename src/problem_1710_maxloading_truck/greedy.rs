pub struct Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        let mut res = 0;

        box_types.sort_by_key(|item| item[1]);
        box_types.reverse();

        for v in box_types {
            let num = *v.get(0).unwrap();
            let unitsize = *v.get(1).unwrap();
            if num <= truck_size {
                res += num * unitsize;
            } else {
                res += truck_size * unitsize;
                break;
            }
            truck_size -= num;
        }

        res
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        Self::maximum_units(box_types, truck_size)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}