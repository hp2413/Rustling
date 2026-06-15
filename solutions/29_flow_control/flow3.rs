// flow3.rs
//
// Solution: `break 'search;` targets the labelled outer loop, so finding the
// first matching pair leaves both loops immediately (a plain `break` would only
// leave the inner loop and keep scanning, overwriting the answer).

fn first_factor_pair(n: u32, target: u32) -> Option<(u32, u32)> {
    let mut found = None;

    'search: for i in 1..=n {
        for j in 1..=n {
            if i * j == target {
                found = Some((i, j));
                break 'search;
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
        assert_eq!(first_factor_pair(6, 6), Some((1, 6)));
        assert_eq!(first_factor_pair(6, 12), Some((2, 6)));
    }

    #[test]
    fn reports_no_pair() {
        assert_eq!(first_factor_pair(3, 100), None);
    }
}
