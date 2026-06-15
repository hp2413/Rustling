// iterators6.rs
//
// Rust gives you three ways to turn a collection into an iterator, and the one
// you pick decides BOTH what each step yields AND what happens to the
// collection afterwards:
//
//   * `iter()`      yields shared references `&T`     — only borrows, so the
//                   collection stays usable after the loop.
//   * `iter_mut()`  yields mutable references `&mut T` — borrows mutably so you
//                   can change elements in place (dereference with `*`).
//   * `into_iter()` yields owned values `T`           — CONSUMES the collection;
//                   it is moved and can no longer be used afterwards.
// (NOTES "Iterators — which to choose"; TRPL ch13.2)
//
// TODO: Complete the three functions below, each using the iterator method
// named in its comment.
//
// Run with `rustlings run iterators6`, get a hint with `rustlings hint iterators6`.

// Sum the numbers WITHOUT taking ownership, so the caller keeps its slice.
fn sum_borrowed(numbers: &[i32]) -> i32 {
    // TODO: Use `.iter()` (which yields `&i32`) and `.sum()`.
}

// Add 1 to every element IN PLACE. Because you mutate through a `&mut i32`,
// remember to dereference with `*` before assigning.
fn add_one_in_place(numbers: &mut [i32]) {
    // TODO: Loop over `numbers.iter_mut()` and do `*n += 1` for each element.
}

// Consume the vector, turning each owned `String` into its length and summing
// those lengths. `into_iter()` MOVES the `String`s out, so no cloning is needed.
fn total_len(words: Vec<String>) -> usize {
    // TODO: Use `.into_iter()`, map each owned `String` to its `.len()`, sum.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_only_borrows() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(sum_borrowed(&v), 10);
        // `v` is still usable here because `sum_borrowed` only borrowed it.
        assert_eq!(v.len(), 4);
    }

    #[test]
    fn iter_mut_changes_in_place() {
        let mut v = vec![1, 2, 3];
        add_one_in_place(&mut v);
        assert_eq!(v, [2, 3, 4]);
    }

    #[test]
    fn into_iter_consumes() {
        let words = vec![String::from("hello"), String::from("world!")];
        assert_eq!(total_len(words), 11);
    }
}
