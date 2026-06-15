// iterators8.rs
//
// Solution: adapters (`zip`/`enumerate`/`filter`/`map`) are lazy and chain
// together; `collect` (with a turbofish when needed) materialises them into a
// `Vec` or `HashMap`, and `partition` splits one iterator into two collections.

use std::collections::HashMap;

fn pair_up(names: &[&str], scores: &[i32]) -> HashMap<String, i32> {
    names
        .iter()
        .zip(scores.iter())
        .map(|(name, score)| (name.to_string(), *score))
        .collect::<HashMap<_, _>>()
}

fn shout_even_positions(words: &[&str]) -> Vec<String> {
    words
        .iter()
        .enumerate()
        .filter(|&(i, _)| i % 2 == 0)
        .map(|(_, w)| w.to_uppercase())
        .collect()
}

fn split_even_odd(numbers: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    numbers.into_iter().partition(|n| n % 2 == 0)
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
