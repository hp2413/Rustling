// move_threads1.rs
//
// A spawned thread might keep running after the function that created it has
// returned. Because of that, a closure passed to `thread::spawn` is required to
// be `'static`: it may NOT borrow local variables, since those could be gone by
// the time the thread reads them. The fix is the `move` keyword, which makes the
// closure take *ownership* of everything it captures. After moving a value into
// a thread, the parent can no longer use it — ownership has been transferred.
// (NOTES "Using move Closures with Threads"; TRPL ch16.1)
//
// TODO: The thread below borrows `numbers`, which the compiler rejects. Add the
// `move` keyword so the closure owns `numbers`, then `join` the handle to get
// its result back.
//
// Run with `rustlings run move_threads1`, get a hint with
// `rustlings hint move_threads1`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::thread;

    #[test]
    fn move_vec_into_thread() {
        let numbers: Vec<i32> = (1..=5).collect();

        // TODO: This closure must OWN `numbers` because the thread may outlive
        // this function. Turn `|| ...` into `move || ...`.
        let handle = thread::spawn(|| numbers.iter().sum::<i32>());

        // `join` waits for the thread to finish and returns whatever the closure
        // returned, wrapped in a `Result`.
        let total = handle.join().unwrap();

        assert_eq!(total, 15);
    }
}
