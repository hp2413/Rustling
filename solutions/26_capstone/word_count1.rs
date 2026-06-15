// word_count1.rs
//
// Solution: iterate the lines and keep the ones that match. The returned
// slices borrow into `contents`, which the `'a` lifetime makes explicit.

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
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
        assert_eq!(search(query, CONTENTS), vec!["safe, fast, productive."]);
    }

    #[test]
    fn case_matters() {
        assert_eq!(search("Rust", CONTENTS).len(), 1);
        assert_eq!(search_case_insensitive("Rust", CONTENTS).len(), 2);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(
            search_case_insensitive(query, CONTENTS),
            vec!["Rust:", "Trust me."],
        );
    }
}
