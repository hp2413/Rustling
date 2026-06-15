// flow1.rs
//
// `loop` repeats a block forever — until something inside it `break`s. Unlike
// `while` and `for`, a `loop` is also an *expression*: you can give `break` a
// value, and that value becomes the result of the whole `loop`. This lets you
// retry until a condition holds and then hand back a result:
//
//     let answer = loop {
//         // ...
//         break 42; // `answer` becomes 42
//     };
//
// (TRPL ch3.5 "Returning Values from Loops")
//
// TODO: Make `loop_value` return 20 by breaking out of the loop with the right
// value once the counter reaches 10.
//
// Run with `rustlings run flow1`, get a hint with `rustlings hint flow1`.

fn loop_value() -> i32 {
    let mut counter = 0;

    // The whole `loop { ... }` is the function's return value here, so whatever
    // you `break` with becomes what `loop_value` returns.
    loop {
        counter += 1;

        if counter == 10 {
            // TODO: Break out of the loop, handing back `counter * 2` as the
            // loop's value (replace `???`).
            break???;
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
