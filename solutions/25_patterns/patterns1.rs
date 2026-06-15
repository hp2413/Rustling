// patterns1.rs
//
// Solution: a guard (`n if n < 0`) handles negatives and an inclusive range
// (`1..=9`) handles the single digits; `_` catches everything else.

fn classify(n: i32) -> &'static str {
    match n {
        n if n < 0 => "negative",
        0 => "zero",
        1..=9 => "single digit",
        _ => "big",
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_numbers() {
        assert_eq!(classify(-5), "negative");
        assert_eq!(classify(0), "zero");
        assert_eq!(classify(1), "single digit");
        assert_eq!(classify(9), "single digit");
        assert_eq!(classify(10), "big");
        assert_eq!(classify(1000), "big");
    }
}
