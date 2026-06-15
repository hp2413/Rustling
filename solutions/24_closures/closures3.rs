// closures3.rs
//
// Solution: the closure mutates the captured `count`, so it is an `FnMut`
// closure and its binding must be `mut`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn counts_up() {
        let mut count = 0;

        let mut increment = || count += 1;

        increment();
        increment();
        increment();

        assert_eq!(count, 3);
    }
}
