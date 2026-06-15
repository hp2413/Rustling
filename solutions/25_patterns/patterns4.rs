// patterns4.rs
//
// Solution: each function uses one of the single-pattern forms — `if let`,
// `let ... else`, and `while let`.

fn double_or_zero(opt: Option<i32>) -> i32 {
    if let Some(n) = opt { n * 2 } else { 0 }
}

fn parse_plus_one(s: &str) -> i32 {
    let Ok(n) = s.parse::<i32>() else {
        return -1;
    };
    n + 1
}

fn drain_sum(mut stack: Vec<i32>) -> i32 {
    let mut total = 0;
    while let Some(top) = stack.pop() {
        total += top;
    }
    total
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn if_let_doubles_or_zero() {
        assert_eq!(double_or_zero(Some(21)), 42);
        assert_eq!(double_or_zero(None), 0);
    }

    #[test]
    fn let_else_parses_or_exits() {
        assert_eq!(parse_plus_one("41"), 42);
        assert_eq!(parse_plus_one("not a number"), -1);
    }

    #[test]
    fn while_let_drains_stack() {
        assert_eq!(drain_sum(vec![1, 2, 3, 4]), 10);
        assert_eq!(drain_sum(vec![]), 0);
    }
}
