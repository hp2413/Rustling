// generics4.rs
//
// A generic function may only use operations the compiler KNOWS every possible
// `T` supports. You promise those capabilities with trait BOUNDS. Comparing
// values with `>` requires `PartialOrd`; copying them out of a slice requires
// `Copy`. When bounds pile up, a `where` clause keeps the signature readable:
//
//   fn largest<T>(list: &[T]) -> T
//   where
//       T: PartialOrd + Copy,
//   { ... }
//
// (NOTES "largest" example; TRPL ch10.2 "Trait Bounds")
//
// TODO: Add the trait bounds `largest` needs, written as a `where` clause.
//
// Run with `rustlings run generics4`, get a hint with `rustlings hint generics4`.

// TODO: This won't compile yet. The body uses `>` (needs `PartialOrd`) and
// copies elements out of the slice with `let mut largest = list[0]` and
// `for &item in list` (needs `Copy`), but bare `T` promises neither. Add a
// `where T: PartialOrd + Copy` clause between the return type and the `{`.
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_number() {
        assert_eq!(largest(&[34, 50, 25, 100, 65]), 100);
    }

    #[test]
    fn largest_char() {
        assert_eq!(largest(&['y', 'm', 'a', 'q']), 'y');
    }
}
