// drop1.rs
//
// Solution: drop `a` early with `std::mem::drop(a)` so it records first. The
// remaining guards `_b` and `_c` then drop automatically at the end of the
// block in reverse declaration order (c before b), giving ["a", "c", "b"].

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
            let _b = Guard::new("b", &log);
            let _c = Guard::new("c", &log);

            std::mem::drop(a);
        }

        assert_eq!(*log.borrow(), ["a", "c", "b"]);
    }
}
