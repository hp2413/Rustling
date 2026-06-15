// formatting1.rs
//
// Solution: each placeholder uses a different feature of the format mini-
// language — positional indices, named arguments, precision, width/alignment,
// and the `Debug` (`{:?}`) format.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn positional_arguments() {
        let s = format!("{1} {0}", "world", "hello");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn named_arguments() {
        let s = format!("Hi, {name}!", name = "Ferris");
        assert_eq!(s, "Hi, Ferris!");
    }

    #[test]
    fn precision() {
        let value = 1.23456;
        let s = format!("{:.2}", value);
        assert_eq!(s, "1.23");
    }

    #[test]
    fn width_and_alignment() {
        let s = format!("{:>8}", 42);
        assert_eq!(s, "      42");
    }

    #[test]
    fn debug_output() {
        let v = vec![1, 2, 3];
        let s = format!("{:?}", v);
        assert_eq!(s, "[1, 2, 3]");
    }
}
