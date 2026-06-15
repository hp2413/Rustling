// drop1.rs
//
// Rust has no garbage collector. Instead it uses RAII: when a value goes out of
// scope, Rust automatically runs its destructor. You hook into that by
// implementing the `Drop` trait — its `drop` method runs the moment the value
// is dropped. Two rules matter here:
//   1. Within a scope, values are dropped in the *reverse* of their declaration
//      order (last declared, first dropped).
//   2. You can drop a value *early* by passing it to `std::mem::drop(value)`.
// (TRPL ch15.3; RBE "RAII")
//
// Each `Guard` records its label into a shared log when it is dropped, so the
// test can observe the exact drop order. (The `Rc<RefCell<...>>` log is just
// plumbing to record that order; `refcell1` covers that type properly.)
//
// The guards are declared in the order a, b, c. Left alone, scope-end order is
// the reverse: c, then b, then a. But the test wants the log to end up as
// ["a", "c", "b"].
//
// TODO: Add exactly ONE `std::mem::drop(...)` call at the marked spot so the
// recorded order becomes ["a", "c", "b"]. (Which guard must be dropped early to
// appear first, while the remaining two still drop in reverse order at the end?)
//
// Run with `rustlings run drop1`, get a hint with `rustlings hint drop1`.

use std::cell::RefCell;
use std::rc::Rc;

struct Guard {
    label: &'static str,
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl Guard {
    fn new(label: &'static str, log: &Rc<RefCell<Vec<&'static str>>>) -> Guard {
        Guard {
            label,
            log: Rc::clone(log),
        }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.log.borrow_mut().push(self.label);
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_order_matches() {
        let log = Rc::new(RefCell::new(Vec::new()));

        {
            let a = Guard::new("a", &log);
            // `_b` and `_c` are dropped implicitly at the end of this block, in
            // reverse declaration order (c before b). They are underscore-named
            // only to silence the "unused variable" warning — their `Drop` still
            // runs.
            let _b = Guard::new("b", &log);
            let _c = Guard::new("c", &log);

            // TODO: Add a single `std::mem::drop(...)` here so the final order is
            // ["a", "c", "b"]. Right now nothing is dropped early, so the block
            // ends with order ["c", "b", "a"].
        }

        assert_eq!(*log.borrow(), ["a", "c", "b"]);
    }
}
