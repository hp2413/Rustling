// weak1.rs
//
// `Rc<T>` keeps data alive as long as at least one *strong* reference exists. If
// two values own each other with `Rc` — a parent owning its child AND the child
// owning its parent right back — their strong counts never reach zero, so the
// memory is never freed. That is a reference CYCLE (a leak).
//
// `Weak<T>` breaks the cycle. A weak reference (created with `Rc::downgrade`)
// does NOT keep the value alive; it only points at it. Because the target might
// already be gone, you can't get a `&T` straight away — you call `.upgrade()`,
// which returns `Option<Rc<T>>`: `Some` if the value still lives, `None` if it
// has been dropped.
//
// Rule of thumb: a parent should *own* its children with `Rc`, but a child
// should only *refer back* to its parent with `Weak`. (TRPL ch15.6)
//
// TODO: Complete the two marked spots so the parent <-> child link works without
// leaking.
//
// Run with `rustlings run weak1`, get a hint with `rustlings hint weak1`.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    value: i32,
    // The back-edge to the parent. It must NOT be a strong `Rc`, or parent and
    // child would keep each other alive forever. `RefCell` lets us fill it in
    // after the parent already exists.
    parent: RefCell<Weak<Node>>,
    // Strong ownership of the children, behind a `RefCell` so we can push more.
    children: RefCell<Vec<Rc<Node>>>,
}

// Attach `child` under `parent`: the parent owns the child strongly, while the
// child points back at the parent weakly.
fn link(parent: &Rc<Node>, child: &Rc<Node>) {
    // The parent strongly owns the child.
    parent.children.borrow_mut().push(Rc::clone(child));

    // TODO: Set the child's `parent` field to a WEAK reference to `parent`.
    // Turn the strong `Rc` into a `Weak` with `Rc::downgrade(parent)` and store
    // it through the `RefCell`: `*child.parent.borrow_mut() = ...;`.
    ???
}

// Return the value of `child`'s parent, if that parent is still alive.
fn parent_value(child: &Rc<Node>) -> Option<i32> {
    // TODO: `child.parent` is a `RefCell<Weak<Node>>`. Borrow it, call
    // `.upgrade()` to turn the `Weak` into an `Option<Rc<Node>>`, then
    // `.map(|parent| parent.value)` to read the value out.
    ???
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

        // Before linking, the leaf has no parent.
        assert_eq!(parent_value(&leaf), None);

        link(&branch, &leaf);

        // Now the weak back-reference upgrades to the branch.
        assert_eq!(parent_value(&leaf), Some(5));
    }

    #[test]
    fn weak_does_not_keep_parent_alive() {
        let leaf = new_node(3);

        {
            let branch = new_node(5);
            link(&branch, &leaf);

            // `branch` is the ONLY strong owner of the parent node; the leaf
            // holds it only weakly, so the strong count stays at 1.
            assert_eq!(Rc::strong_count(&branch), 1);
            assert_eq!(parent_value(&leaf), Some(5));
        }

        // `branch` has been dropped. Because the back-edge was weak, the parent
        // node was freed and the weak reference can no longer upgrade.
        assert_eq!(parent_value(&leaf), None);
    }
}
