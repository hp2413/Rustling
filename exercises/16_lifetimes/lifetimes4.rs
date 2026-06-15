// lifetimes4.rs
//
// So far we've annotated lifetimes only on functions, but structs need them too
// whenever they *hold* a reference instead of an owned value. The annotation
// `struct Excerpt<'a> { part: &'a str }` tells the compiler that an `Excerpt`
// borrows the `&str` it stores in `part`, so the struct may not outlive the data
// it points at. Without the lifetime parameter the compiler can't reason about
// how long that borrow is valid and refuses to compile.
// (NOTES "Struct with lifetimes"; TRPL ch10.3)
//
// TODO: The struct below stores a string slice but is missing its lifetime
// parameter, so it won't compile. Add a lifetime parameter `'a` to the struct
// and tie the `part` field's reference to it (`part: &'a str`).
//
// Run with `rustlings run lifetimes4`, get a hint with `rustlings hint lifetimes4`.

// TODO: Add the lifetime parameter to the struct and use it on the field.
struct Excerpt {
    part: &str,
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
