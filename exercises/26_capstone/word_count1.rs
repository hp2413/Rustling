// word_count1.rs
//
// Capstone (1/2): a miniature version of TRPL ch12's `minigrep`. The real
// program reads a query and a file from the command line; here we strip away
// the I/O and keep the heart of it — the *search* logic — so it can be driven
// entirely by tests.
//
// The key idea is lifetimes: `search` returns slices that borrow *into*
// `contents`, so the returned `Vec<&str>` may not outlive `contents`. We tie
// them together with the lifetime `'a`. (`query` is unrelated, so it needs no
// named lifetime.) See TRPL ch12.3–12.4.
//
// TODO: Implement both functions so the tests pass. Neither does any work yet.
//
// Run with `rustlings run word_count1`, get a hint with `rustlings hint word_count1`.

// Case-sensitive search: return every line of `contents` that contains `query`.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // TODO: Iterate over `contents.lines()`. For each `line`, keep it (push it
    // into `results`) when `line.contains(query)` is true.
    results
}

// Case-insensitive search: same as `search`, but "RUST" should match "rust".
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // Lowercasing produces a brand-new `String`, so compare each line's
    // lowercased copy against this.
    let query = query.to_lowercase();
    let mut results = Vec::new();
    // TODO: Iterate over `contents.lines()`. Keep each `line` whose
    // `line.to_lowercase()` `contains(&query)`. (Push the ORIGINAL `line`, not
    // the lowercased copy — the lowercased `String` is a temporary.)
    results
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENTS: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    #[test]
    fn case_sensitive() {
        let query = "duct";
        // Only "productive" contains "duct"; "Rust"/"Trust" do not.
        assert_eq!(search(query, CONTENTS), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_matters() {
        // "Rust" (capital R) only matches the "Rust:" line case-sensitively...
        assert_eq!(search("Rust", CONTENTS).len(), 1);
        // ...but once case is ignored it also matches "Trust me.".
        assert_eq!(search_case_insensitive("Rust", CONTENTS).len(), 2);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        // Matches "Rust:" and "Trust me." (case-insensitively).
        assert_eq!(
            search_case_insensitive(query, CONTENTS),
            vec!["Rust:", "Trust me."],
        );
    }
}
