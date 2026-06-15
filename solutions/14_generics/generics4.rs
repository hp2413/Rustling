// generics4.rs
//
// Solution: constrain `T` with a `where` clause so the body's operations are
// allowed — `PartialOrd` to compare with `>`, and `Copy` to move elements out
// of the slice by value.

fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
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
