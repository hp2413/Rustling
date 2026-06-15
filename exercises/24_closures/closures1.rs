// closures1.rs
//
// A closure is an anonymous function that you can store in a variable or pass
// to another function. The syntax is `|parameters| body`, for example
// `|a, b| a + b`. Unlike a `fn` item, a closure can usually *infer* its
// parameter and return types from how you call it, so you rarely annotate them.
// (TRPL ch13.1 "Closures")
//
// TODO: Replace `???` with a closure that adds its two arguments together.
//
// Run with `rustlings run closures1`, get a hint with `rustlings hint closures1`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn call_closure() {
        // TODO: Write a closure of the form `|a, b| a + b` here. The types of
        // `a` and `b` are inferred from the calls below, so you don't need to
        // annotate them.
        let add = ???;

        assert_eq!(add(2, 3), 5);
        assert_eq!(add(10, 20), 30);
    }
}
