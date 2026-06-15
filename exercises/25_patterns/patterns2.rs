// patterns2.rs
//
// Patterns can reach *inside* a value and pull its parts out into new
// variables. You can destructure structs (`Point { x, y }`), tuples
// (`(a, b)`), and enum variants — and you can nest these, e.g. an enum variant
// that contains a struct or a tuple. Because the `Command` enum below has only
// three variants, matching all three is already exhaustive (no `_` needed).
// (TRPL "Pattern syntax".)
//
// TODO: Replace each `???` with a pattern that destructures the data carried by
// the enum variant, binding the names used in that arm's body.
//
// Run with `rustlings run patterns2`, get a hint with `rustlings hint patterns2`.

struct Point {
    x: i32,
    y: i32,
}

enum Command {
    Move(Point),
    Scale((i32, i32)),
    Stop,
}

fn run(command: Command) -> String {
    match command {
        // TODO: Destructure the nested `Point` so `x` and `y` are bound here.
        Command::Move(???) => format!("move to ({x}, {y})"),
        // TODO: Destructure the nested tuple so `w` and `h` are bound here.
        Command::Scale(???) => format!("scale by {w}x{h}"),
        Command::Stop => "stop".to_string(),
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runs_commands() {
        assert_eq!(run(Command::Move(Point { x: 3, y: 4 })), "move to (3, 4)");
        assert_eq!(run(Command::Scale((2, 5))), "scale by 2x5");
        assert_eq!(run(Command::Stop), "stop");
    }
}
