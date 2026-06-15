// traits6.rs
//
// Solution: give `summarize` a default body in the trait. `Tweet` overrides it,
// while `Article`'s empty `impl` block inherits the default.

trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

struct Article;

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
