// derive2.rs
//
// More derivable traits, for three very common needs:
//
//   * `Default` — gives `T::default()` / `Default::default()`, filling every
//     field with ITS default (0, empty `String`, etc.).
//   * `PartialOrd` + `Ord` — let values be ORDERED, so you can `.sort()` a
//     `Vec<T>` or call `.min()` / `.max()`. `Ord` requires `Eq`, and ordering is
//     compared field-by-field in declaration order.
//   * `Eq` + `Hash` — required to use a type as a `HashMap` / `HashSet` KEY.
//
// (`PartialEq` you met in derive1; `Eq` just adds the promise that every value
// equals itself, which both `Ord` and `Hash` rely on.)
//
// TODO: Add a single `#[derive(...)]` line above `Version` so the tests pass.
// Read the tests to see which capabilities are exercised: built with
// `Default::default()`, sorted with `.sort()`, and used as a `HashMap` key.
//
// Run with `rustlings run derive2`, get a hint with `rustlings hint derive2`.

use std::collections::HashMap;

// TODO: Add the derive attribute here. Fields are compared in declaration
// order, so `major` outranks `minor` — exactly what version sorting wants.
struct Version {
    major: u32,
    minor: u32,
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_fills_zeroes() {
        let v: Version = Default::default();
        assert_eq!(v, Version { major: 0, minor: 0 });
    }

    #[test]
    fn versions_sort_in_order() {
        let mut versions = vec![
            Version { major: 1, minor: 4 },
            Version { major: 1, minor: 2 },
            Version { major: 0, minor: 9 },
        ];
        versions.sort();
        assert_eq!(
            versions,
            vec![
                Version { major: 0, minor: 9 },
                Version { major: 1, minor: 2 },
                Version { major: 1, minor: 4 },
            ]
        );
    }

    #[test]
    fn version_works_as_hashmap_key() {
        let mut names = HashMap::new();
        names.insert(Version { major: 1, minor: 0 }, "stable");
        names.insert(Version { major: 2, minor: 0 }, "next");
        assert_eq!(names.get(&Version { major: 1, minor: 0 }), Some(&"stable"));
        assert_eq!(names.get(&Version { major: 2, minor: 0 }), Some(&"next"));
    }
}
