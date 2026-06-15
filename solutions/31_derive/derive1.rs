// derive1.rs
//
// Solution: one derive line gives `Point` all four behaviours the tests need —
// `Debug` for `{:?}`, `Clone` + `Copy` so it copies instead of moving, and
// `PartialEq` for `==` / `!=`.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_lets_value_be_used_after_assignment() {
        let a = Point { x: 1, y: 2 };
        let b = a;
        assert_eq!(a.x, 1);
        assert_eq!(b.y, 2);
    }

    #[test]
    fn clone_duplicates_a_collection() {
        let points = vec![Point { x: 3, y: 4 }, Point { x: 5, y: 6 }];
        let copy = points.clone();
        assert_eq!(copy, points);
    }

    #[test]
    fn partial_eq_enables_comparison() {
        let a = Point { x: 1, y: 2 };
        let b = Point { x: 1, y: 2 };
        let c = Point { x: 9, y: 9 };
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn debug_enables_formatting() {
        let a = Point { x: 1, y: 2 };
        assert_eq!(format!("{a:?}"), "Point { x: 1, y: 2 }");
    }
}
