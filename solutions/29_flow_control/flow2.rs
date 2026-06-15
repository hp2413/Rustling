// flow2.rs
//
// Solution: a plain `while n > 0` condition for the countdown, and
// `while let Some(top) = stack.pop()` to loop until the stack is empty.

fn countdown(mut n: u32) -> Vec<u32> {
    let mut out = Vec::new();

    while n > 0 {
        out.push(n);
        n -= 1;
    }

    out
}

fn drain_stack(mut stack: Vec<i32>) -> Vec<i32> {
    let mut out = Vec::new();

    while let Some(top) = stack.pop() {
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
        assert_eq!(drain_stack(vec![1, 2, 3]), vec![3, 2, 1]);
    }
}
