// weak1.rs
//
// Solution: store the parent back-edge as a `Weak` (via `Rc::downgrade`) so it
// doesn't keep the parent alive, and read it back with `.upgrade()`, which gives
// `Some(Rc<Node>)` while the parent lives and `None` after it is dropped.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn link(parent: &Rc<Node>, child: &Rc<Node>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

fn parent_value(child: &Rc<Node>) -> Option<i32> {
    child.parent.borrow().upgrade().map(|parent| parent.value)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_node(value: i32) -> Rc<Node> {
        Rc::new(Node {
            value,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    #[test]
    fn child_can_reach_parent() {
        let leaf = new_node(3);
        let branch = new_node(5);

        assert_eq!(parent_value(&leaf), None);

        link(&branch, &leaf);

        assert_eq!(parent_value(&leaf), Some(5));
    }

    #[test]
    fn weak_does_not_keep_parent_alive() {
        let leaf = new_node(3);

        {
            let branch = new_node(5);
            link(&branch, &leaf);

            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(parent_value(&leaf), Some(5));
        }

        assert_eq!(parent_value(&leaf), None);
    }
}
