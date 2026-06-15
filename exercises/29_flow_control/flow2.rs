// flow2.rs
//
// `while <condition>` repeats its body as long as the condition stays `true`.
// `while let <pattern> = <expr>` is a cousin that keeps looping as long as the
// value still matches the pattern — perfect for draining something like a stack
// with `.pop()`, which yields `Some(x)` until it's empty and then `None`:
//
//     while let Some(x) = stack.pop() {
//         // runs once per element, until `pop()` returns `None`
//     }
//
// (RBE "while" / "while let")
//
// TODO: Fix the two loop headers below (the `???`s) so the tests pass.
//
// Run with `rustlings run flow2`, get a hint with `rustlings hint flow2`.

fn countdown(mut n: u32) -> Vec<u32> {
    let mut out = Vec::new();

    // TODO: Keep looping while `n` is greater than 0.
    while ??? {
        out.push(n);
        n -= 1;
    }

    out
}

fn drain_stack(mut stack: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();

    // TODO: Use `while let` to pop from `stack` until it is empty, binding each
    // popped value to `top`.
    while ??? {
        out.push(top);
    }

    out
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn while_counts_down() {
        assert_eq!(countdown(3), vec![3, 2, 1]);
        assert_eq!(countdown(0), Vec::<u32>::new());
    }

    #[test]
    fn while_let_drains_stack() {
        // `pop` removes from the end, so the stack comes out reversed.
        assert_eq!(drain_stack(vec![1, 2, 3]), vec![3, 2, 1]);
    }
}
