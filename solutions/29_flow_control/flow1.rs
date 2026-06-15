// flow1.rs
//
// Solution: `break <value>;` makes that value the result of the `loop`
// expression. Here the `loop` is the function's tail expression, so its value
// becomes the function's return value.

fn loop_value() -> i32 {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loop_returns_doubled_counter() {
        assert_eq!(loop_value(), 20);
    }
}
