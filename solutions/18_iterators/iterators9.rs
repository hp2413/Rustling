// iterators9.rs
//
// Solution: implementing `Iterator` is just `type Item = u32;` plus a `next`
// that drives the state forward and returns `None` when done. Every adapter
// (`zip`, `map`, `filter`, `skip`, `sum`, ...) is then available for free.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_from_one_to_five() {
        let collected: Vec<u32> = Counter::new().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn next_yields_each_value_then_none() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn adapters_come_for_free() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
}
