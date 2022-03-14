use std::collections::VecDeque;

pub struct RecentCounter {
    queue: VecDeque<i32>
}

impl RecentCounter {
    fn new() -> Self {
        Self { queue: VecDeque::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push_front(t);
        while let Some(&back) = self.queue.back() {
            if back < t - 3000 {
                self.queue.pop_back();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::RecentCounter for RecentCounter {
    fn new() -> Self {
        Self::new()
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.ping(t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::RecentCounter>();
    }
}
