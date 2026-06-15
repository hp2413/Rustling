// move_threads1.rs
//
// Solution: add `move` so the closure takes ownership of `numbers` and can
// safely outlive this function; `join` returns the computed sum.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn move_vec_into_thread() {
        let numbers: Vec<i32> = (1..=5).collect();

        let handle = thread::spawn(move || numbers.iter().sum::<i32>());

        let total = handle.join().unwrap();

        assert_eq!(total, 15);
    }
}
