// closures2.rs
//
// Solution: the closure borrows `numbers` (no `move`), so it can be summed and
// `numbers` is still usable afterwards.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn closure_borrows_by_default() {
        let numbers: Vec<i32> = (1..=4).collect();

        let sum_all = || numbers.iter().sum::<i32>();

        assert_eq!(sum_all(), 10);

        assert_eq!(numbers.len(), 4);
    }
}
