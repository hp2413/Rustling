// closures1.rs
//
// Solution: a closure is written `|parameters| body`. The parameter and return
// types are inferred from how the closure is called.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_closure() {
        let add = |a, b| a + b;

        assert_eq!(add(2, 3), 5);
        assert_eq!(add(10, 20), 30);
    }
}
