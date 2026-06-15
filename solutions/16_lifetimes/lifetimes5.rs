// lifetimes5.rs
//
// Solution: because `longest` could return either argument, tie both parameters
// and the return type to a single lifetime `'a`. This says the result borrows
// from the same scope as the inputs, so it can't outlive either of them.
// `first_word` needs no annotation thanks to lifetime elision (one reference in,
// one reference out).

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_picks_the_longer() {
        assert_eq!(longest("abcd", "xy"), "abcd");
        assert_eq!(longest("a", "longer"), "longer");
    }

    #[test]
    fn elision_still_works() {
        assert_eq!(first_word("hello world"), "hello");
    }
}
