// slices1.rs
//
// A slice is a *borrowed view* into a contiguous run of elements — it does not
// own or copy the data. You create one with a range index: `&v[1..4]` borrows
// elements 1, 2 and 3 (the end index is exclusive). Because both arrays and
// vectors can be viewed as slices, functions should usually accept `&[T]`
// rather than `&Vec<T>` or `&[T; N]` — that way they work with both.
// (TRPL ch4.3 "The Slice Type")
//
// TODO: Build the slices requested below (replace the `???`s) so the tests pass.
//
// Run with `rustlings run slices1`, get a hint with `rustlings hint slices1`.

fn sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_the_middle() {
        let v: Vec<i32> = (1..=5).map(|n| n * 10).collect(); // [10, 20, 30, 40, 50]
        // TODO: Make `middle` borrow just the three middle elements
        // (20, 30, 40) of `v`, using a range index like `&v[start..end]`.
        let middle = ???;
        assert_eq!(sum_slice(middle), 90);
    }

    #[test]
    fn works_on_arrays_too() {
        let arr = [1, 2, 3, 4];
        // TODO: Pass the whole array to `sum_slice` as a slice (an array
        // coerces to a slice with `&arr` or `&arr[..]`).
        assert_eq!(sum_slice(???), 10);
    }
}
