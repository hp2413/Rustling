// lifetimes4.rs
//
// Solution: a struct that stores a reference needs a lifetime parameter. Declare
// it after the struct name (`Excerpt<'a>`) and use it on the borrowed field
// (`part: &'a str`). The struct is now valid only as long as the data it borrows.

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excerpt_borrows_a_slice() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        // Borrow the first sentence out of `novel`.
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let excerpt = Excerpt {
            part: first_sentence,
        };

        assert_eq!(excerpt.part, "Call me Ishmael");
    }
}
