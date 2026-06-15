// iterators7.rs
//
// Some iterator methods CONSUME the whole iterator to produce a single answer,
// while adapters like `scan` produce a NEW iterator of intermediate results:
//
//   * `fold(init, |acc, x| ...)` threads an accumulator through every element
//     and returns the final accumulator.
//   * `product()` multiplies all elements together (and `sum()` adds them up);
//     both are just specialised folds.
//   * `scan(init, |state, x| Some(...))` is like `fold`, but it YIELDS each
//     intermediate value, giving you a running total / running product.
// (NOTES "consuming adapters" table; TRPL ch13.2)
//
// TODO: Complete the three functions below using the method named in each.
//
// Run with `rustlings run iterators7`, get a hint with `rustlings hint iterators7`.

// Factorial with `fold`: multiply 1 * 2 * ... * n, starting the accumulator at 1.
fn factorial_fold(n: u64) -> u64 {
    // TODO: `(1..=n).fold(1, |acc, x| ...)`. Note: an empty range (when n == 0)
    // never runs the closure, so `fold` returns the initial value `1`.
}

// Factorial again, but with the dedicated `product` consumer.
fn factorial_product(n: u64) -> u64 {
    // TODO: `(1..=n).product()`. The product of an empty range is `1`.
}

// Running sums: given [a, b, c], return [a, a+b, a+b+c] using `scan`.
fn running_totals(numbers: &[i32]) -> Vec<i32> {
    // TODO: `numbers.iter().scan(0, |state, &x| { *state += x; Some(*state) })`
    // then `.collect()`. `scan` keeps `state` between steps and yields it.
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
