// iterators7.rs
//
// Solution: `fold` collapses an iterator to one value by threading an
// accumulator; `product` is a ready-made multiplying fold; `scan` is the lazy
// adapter that yields every intermediate accumulator value.

// We intentionally use `fold` here to teach it, even though clippy correctly
// notes that this exact computation is what the dedicated `product()` consumer
// (used in `factorial_product` below) is for.
#[allow(clippy::unnecessary_fold)]
fn factorial_fold(n: u64) -> u64 {
    (1..=n).fold(1, |acc, x| acc * x)
}

fn factorial_product(n: u64) -> u64 {
    (1..=n).product()
}

fn running_totals(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fold_factorial() {
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_fold(5), 120);
    }

    #[test]
    fn product_factorial() {
        assert_eq!(factorial_product(0), 1);
        assert_eq!(factorial_product(5), 120);
    }

    #[test]
    fn scan_running_totals() {
        assert_eq!(running_totals(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_totals(&[]), Vec::<i32>::new());
    }
}
