// iterators6.rs
//
// Solution: `iter()` borrows (`&T`), `iter_mut()` borrows mutably (`&mut T`,
// dereference with `*` to write), and `into_iter()` consumes the collection,
// handing you owned `T` values.

fn sum_borrowed(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn add_one_in_place(numbers: &mut [i32]) {
    for n in numbers.iter_mut() {
        *n += 1;
    }
}

fn total_len(words: Vec<String>) -> usize {
    words.into_iter().map(|w| w.len()).sum()
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
