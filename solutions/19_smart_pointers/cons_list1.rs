// cons_list1.rs
//
// Solution: `match self` in each method. `Nil` is the base case; `Cons` does a
// bit of work and recurses on the boxed tail (`Box<List>` derefs to `List`, so
// `rest.len()` / `rest.sum()` call straight through).

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, rest) => 1 + rest.len(),
        }
    }

    fn sum(&self) -> i32 {
        match self {
            List::Nil => 0,
            List::Cons(value, rest) => value + rest.sum(),
        }
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
