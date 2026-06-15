// iterators9.rs
//
// Every adapter you have used so far (`map`, `filter`, `zip`, `sum`, ...) comes
// from ONE trait: `Iterator`. You can put your own type on the receiving end of
// all of them by implementing it yourself. The trait needs only two things:
//
//   * an associated type `Item` — what each step yields, and
//   * a method `fn next(&mut self) -> Option<Self::Item>` — produce the next
//     value, or `None` once the sequence is finished.
//
// Implement those two and EVERY adapter method comes for free, because the
// standard library defines them all in terms of `next`. (TRPL ch13.2; RBE ch16)
//
// TODO: Make `Counter` yield 1, 2, 3, 4, 5 and then stop, by completing the
// `Iterator` implementation below.
//
// Run with `rustlings run iterators9`, get a hint with `rustlings hint iterators9`.

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // TODO: Each call to `next` yields a `u32`, so set the associated type.
    type Item = ???;

    // TODO: Advance the counter. While `count` is still below 5, increment it
    // and return `Some(self.count)`. Once it has reached 5, every further call
    // must return `None` so the iterator knows it is finished.
    fn next(&mut self) -> Option<Self::Item> {
        ???
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
        // Collecting the iterator runs `next` until it returns `None`.
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
        // The classic TRPL chain: zip this counter with itself shifted by one,
        // multiply each pair, keep the multiples of 3, and sum them.
        // Pairs: (1,2) (2,3) (3,4) (4,5) -> 2, 6, 12, 20 -> keep 6, 12 -> 18.
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
}
