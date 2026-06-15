// closures4.rs
//
// Solution: accept the closure with a generic parameter bound by the `Fn`
// trait that matches its signature, `F: Fn(i32) -> i32`.

fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 {
    f(x)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_closure() {
        assert_eq!(apply(|n| n * 2, 5), 10);
        assert_eq!(apply(|n| n + 100, 1), 101);
    }
}
