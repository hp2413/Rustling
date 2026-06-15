// lifetimes5.rs
//
// When a function returns a reference, the compiler must know which input that
// reference is borrowed from. Often it figures this out on its own through the
// *lifetime elision rules* (e.g. a function with a single reference parameter
// ties the output to that input automatically). But when there are *several*
// reference inputs and any of them could be returned, elision can't guess — you
// must connect them with an explicit lifetime. Tying both inputs and the output
// to the same `'a` tells the compiler the result can't outlive either input.
// (NOTES "Lifetimes"; TRPL ch10.3)
//
// TODO: `longest` returns one of its two `&str` arguments, so the compiler can't
// elide the lifetimes here. Add a lifetime parameter `'a` and apply it to both
// parameters and the return type so this compiles. (Leave `first_word` alone —
// it needs no annotation, demonstrating elision with a single reference input.)
//
// Run with `rustlings run lifetimes5`, get a hint with `rustlings hint lifetimes5`.

// TODO: Add the lifetime annotations so this compiles.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}

// Don't change this function: with a single reference input, lifetime elision
// ties the returned slice to `s` automatically, so no annotation is needed.
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
