// traits6.rs
//
// A trait method can carry a DEFAULT implementation: a body written right in
// the trait. Any type that implements the trait gets that body for free, so an
// empty `impl Trait for Type {}` block already has a working method. A type can
// still OVERRIDE the default by writing its own version. (NOTES "Trait default
// implementation"; TRPL ch10.2)
//
// TODO: Give `summarize` a default body so `Article`'s empty `impl` below
// compiles and falls back to it.
//
// Run with `rustlings run traits6`, get a hint with `rustlings hint traits6`.

trait Summary {
    // TODO: Replace the `;` with a default method body `{ ... }` that returns
    // the `String` "(Read more...)". Without a body this is a *required*
    // method, and `Article`'s empty `impl` block below won't compile.
    fn summarize(&self) -> String;
}

struct Tweet {
    username: String,
    content: String,
}

// `Tweet` OVERRIDES the default with its own version of `summarize`.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

struct Article;

// `Article` provides NO `summarize` of its own, so it INHERITS the trait's
// default — but only once that default actually exists.
impl Summary for Article {}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tweet_overrides_default() {
        let tweet = Tweet {
            username: String::from("rustlang"),
            content: String::from("1.0 is out!"),
        };
        assert_eq!(tweet.summarize(), "@rustlang: 1.0 is out!");
    }

    #[test]
    fn article_uses_default() {
        assert_eq!(Article.summarize(), "(Read more...)");
    }
}
