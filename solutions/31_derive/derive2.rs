// derive2.rs
//
// Solution: one derive line covers every capability the tests need — `Default`
// for `::default()`, `PartialOrd` + `Ord` (which needs `Eq`) for `.sort()`,
// `Eq` + `Hash` to be a `HashMap` key, and `Debug` + `PartialEq` for the
// assertions.

use std::collections::HashMap;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
