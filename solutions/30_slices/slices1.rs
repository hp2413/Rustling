// slices1.rs
//
// Solution: `&v[1..4]` borrows elements at indices 1, 2 and 3 (end exclusive),
// and `&arr` coerces the whole array to a `&[i32]` slice.

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
        let middle = &v[1..4];
        assert_eq!(sum_slice(middle), 90);
    }

    #[test]
    fn works_on_arrays_too() {
        let arr = [1, 2, 3, 4];
        assert_eq!(sum_slice(&arr), 10);
    }
}
