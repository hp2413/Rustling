// patterns2.rs
//
// Solution: destructure the nested data inside each enum variant — a `Point`
// struct for `Move` and a tuple for `Scale` — binding the inner values.

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
        Command::Move(Point { x, y }) => format!("move to ({x}, {y})"),
        Command::Scale((w, h)) => format!("scale by {w}x{h}"),
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
