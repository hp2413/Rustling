// refcell1.rs
//
// Rust's borrowing rules — one mutable borrow XOR any number of shared borrows —
// are normally enforced at compile time. `RefCell<T>` moves those checks to
// RUNTIME instead: from an *immutable* `RefCell` you can still ask for a shared
// borrow with `.borrow()` or an exclusive one with `.borrow_mut()`, and it
// panics if you ever break the rules while the program runs. This trick is
// called "interior mutability": mutating data you only hold a shared reference
// to.
//
// On its own a `RefCell` has a single owner. Wrap it in an `Rc<T>` (which allows
// many owners) to get `Rc<RefCell<T>>`: a value that several owners can share
// AND mutate — every clone of the `Rc` points at the very same `RefCell`.
// (TRPL ch15.5)
//
// TODO: Two things:
//   1. Complete `add_one` so it increments the `i32` inside the `RefCell` by 1.
//   2. Make `another_owner` a second handle to the SAME cell.
//
// Run with `rustlings run refcell1`, get a hint with `rustlings hint refcell1`.

use std::cell::RefCell;
use std::rc::Rc;

fn add_one(cell: &Rc<RefCell<i32>>) {
    // TODO: Get a mutable borrow with `cell.borrow_mut()` and add 1 through it
    // (a mutable borrow derefs to the inner value, so `*... += 1`).
    ???
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_mutation() {
        let counter = Rc::new(RefCell::new(0));

        // TODO: Make `another_owner` a SECOND owner of the same cell by cloning
        // the `Rc` itself (`Rc::clone(&counter)`) — not the inner number.
        let another_owner = ???;

        // Both handles refer to the same RefCell, so these three increments all
        // land on one shared number.
        add_one(&counter);
        add_one(&another_owner);
        add_one(&counter);

        assert_eq!(*counter.borrow(), 3);
        assert_eq!(*another_owner.borrow(), 3);
        // Two `Rc` handles are alive, so the strong count is 2.
        assert_eq!(Rc::strong_count(&counter), 2);
    }
}
