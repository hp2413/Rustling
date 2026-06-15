// cons_list1.rs
//
// `box1` showed *why* a recursive type like the "cons list" must wrap its tail
// in a `Box<List>`: a `Box` is a pointer of a known size, so the compiler can
// size the type even though it is defined in terms of itself. This time the
// `Box` is already in place — your job is to write the *recursive functions*
// that walk the list.
//
// A cons list is either:
//   * `Cons(value, rest)` — a value followed by the rest of the list, or
//   * `Nil`               — the end of the list.
//
// To process one, you `match self`: the `Nil` arm is the base case that stops
// the recursion, and the `Cons` arm does some work and then recurses into the
// boxed tail. Calling a method on the `Box<List>` tail works directly because
// `Box<T>` derefs to `T`. (TRPL ch15.1; RBE "Box, stack and heap")
//
// TODO: Implement `len` and `sum` for `List` so the tests pass. Both must
// `match self`, return the base value for `Nil`, and recurse on the tail for
// `Cons`. Replace the `todo!()` bodies.
//
// Run with `rustlings run cons_list1`, get a hint with `rustlings hint cons_list1`.

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    // TODO: Return how many values the list holds.
    // `Nil` has length 0; `Cons(_, rest)` is `1 + rest.len()`.
    fn len(&self) -> usize {
        todo!()
    }

    // TODO: Return the sum of every value in the list.
    // `Nil` sums to 0; `Cons(value, rest)` is `value + rest.sum()`.
    fn sum(&self) -> i32 {
        todo!()
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;
    use List::{Cons, Nil};

    // Builds the list 1 -> 2 -> 3 -> Nil.
    fn sample() -> List {
        Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))))
    }

    #[test]
    fn empty_list_is_zero() {
        let list = Nil;
        assert_eq!(list.len(), 0);
        assert_eq!(list.sum(), 0);
    }

    #[test]
    fn counts_elements() {
        assert_eq!(sample().len(), 3);
    }

    #[test]
    fn sums_elements() {
        assert_eq!(sample().sum(), 6);
    }
}
