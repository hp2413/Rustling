// closures3.rs
//
// A closure that *changes* a value it captured needs mutable access to that
// value. A closure that mutates its environment implements the `FnMut` trait.
// To call such a closure (and let it mutate) you must store it in a `mut`
// binding, because each call mutates state the closure holds on to.
// (TRPL ch13.1)
//
// TODO: Make `increment` a closure that adds 1 to the captured `count` each
// time it is called, and fix its binding so the closure can mutate and be
// called repeatedly.
//
// Run with `rustlings run closures3`, get a hint with `rustlings hint closures3`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn counts_up() {
        let mut count = 0;

        // TODO: This closure should do `count += 1`. Because it mutates a
        // captured variable, the closure binding itself must be `mut`, i.e.
        // change `let increment` into `let mut increment`.
        let increment = ???;

        increment();
        increment();
        increment();

        assert_eq!(count, 3);
    }
}
