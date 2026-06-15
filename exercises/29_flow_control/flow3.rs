// flow3.rs
//
// `for x in 1..=n` iterates over an *inclusive* range (1, 2, ..., n). When you
// nest loops, a plain `break` only leaves the innermost one. To break out of an
// outer loop from inside an inner loop, give the outer loop a *label* (a name
// starting with a single quote, like `'search:`) and break that label:
//
//     'outer: for i in 0..n {
//         for j in 0..n {
//             if done { break 'outer; } // exits BOTH loops at once
//         }
//     }
//
// (RBE "for and range" / "Nesting and labels")
//
// TODO: The outer loop is already labelled `'search`. Replace `break ???` so it
// breaks out of *both* loops the moment a matching pair is found.
//
// Run with `rustlings run flow3`, get a hint with `rustlings hint flow3`.

// Returns the first `(i, j)` with `1 <= i, j <= n` such that `i * j == target`,
// scanning `i` from the outside and `j` on the inside.
fn first_factor_pair(n: u32, target: u32) -> Option<(u32, u32)> {
    let mut found = None;

    'search: for i in 1..=n {
        for j in 1..=n {
            if i * j == target {
                found = Some((i, j));
                // TODO: Break out of BOTH loops, not just the inner one.
                break???;
            }
        }
    }

    found
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_first_pair() {
        // i goes 1,2,3...; for i=1 the inner loop reaches j=6 first.
        assert_eq!(first_factor_pair(6, 6), Some((1, 6)));
        assert_eq!(first_factor_pair(6, 12), Some((2, 6)));
    }

    #[test]
    fn reports_no_pair() {
        assert_eq!(first_factor_pair(3, 100), None);
    }
}
