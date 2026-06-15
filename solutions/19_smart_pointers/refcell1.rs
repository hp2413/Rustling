// refcell1.rs
//
// Solution: `borrow_mut()` hands out an exclusive borrow of the inner value, so
// `*cell.borrow_mut() += 1` mutates it. `Rc::clone` makes a second owner of the
// same `RefCell` (the strong count becomes 2), so both handles see every
// increment.

use std::cell::RefCell;
use std::rc::Rc;

fn add_one(cell: &Rc<RefCell<i32>>) {
    *cell.borrow_mut() += 1;
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

        let another_owner = Rc::clone(&counter);

        add_one(&counter);
        add_one(&another_owner);
        add_one(&counter);

        assert_eq!(*counter.borrow(), 3);
        assert_eq!(*another_owner.borrow(), 3);
        assert_eq!(Rc::strong_count(&counter), 2);
    }
}
