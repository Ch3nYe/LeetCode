pub struct RecentCounter {
    queue: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    fn ping(&mut self, t: i32) -> i32 {
        self.queue.push(t);
        while t-self.queue[0]>3000 {
            self.queue.remove(0);
        }
        self.queue.len() as i32
    }
}

/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */

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
