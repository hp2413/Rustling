// iterators8.rs
//
// Iterator adapters are LAZY: `zip`, `enumerate`, `filter`, and `map` just
// describe work and do nothing until a consumer runs them. `collect()` is the
// consumer that gathers the results into a collection — and it is general
// enough to build a `Vec`, a `HashMap`, and more. When the target type can't be
// inferred, spell it out with a turbofish: `.collect::<HashMap<_, _>>()`.
// `partition` is a consumer that splits one iterator into TWO collections.
// (NOTES "adapter tables"; TRPL ch13.2)
//
// TODO: Complete the three functions below.
//
// Run with `rustlings run iterators8`, get a hint with `rustlings hint iterators8`.

use std::collections::HashMap;

// Pair each name with its score by ZIPPING the two slices together, then
// COLLECT the pairs into a `HashMap`. Note the turbofish that tells `collect`
// which collection to build.
fn pair_up(names: &[&str], scores: &[i32]) -> HashMap<String, i32> {
    // TODO: `names.iter().zip(scores.iter())`, map each (`&&str`, `&i32`) pair
    // to (`String`, `i32`), then `.collect::<HashMap<_, _>>()`.
}

// Keep only the words at EVEN positions and shout them. Uses `enumerate` (to
// get each index), `filter` (keep even indices), and `map` (uppercase).
fn shout_even_positions(words: &[&str]) -> Vec<String> {
    // TODO: `words.iter().enumerate().filter(|&(i, _)| i % 2 == 0)`
    // then `.map(|(_, w)| w.to_uppercase()).collect()`.
}

// Split the numbers into (evens, odds) in ONE pass with `partition`.
fn split_even_odd(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    // TODO: `numbers.into_iter().partition(|n| n % 2 == 0)`.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zips_into_hashmap() {
        let names = ["alice", "bob"];
        let scores = [90, 85];
        let map = pair_up(&names, &scores);
        assert_eq!(map.get("alice"), Some(&90));
        assert_eq!(map.get("bob"), Some(&85));
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn enumerate_filter_map() {
        let words = ["a", "b", "c", "d", "e"];
        // Indices 0, 2, 4 survive the filter -> "A", "C", "E".
        assert_eq!(shout_even_positions(&words), vec!["A", "C", "E"]);
    }

    #[test]
    fn partitions_even_and_odd() {
        let (evens, odds) = split_even_odd(vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
}
