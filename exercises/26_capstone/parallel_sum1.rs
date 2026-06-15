// parallel_sum1.rs
//
// Capstone (2/2): the NOTES "sum 1..=N using all cores" assignment. Instead of
// summing a big range on one thread, we split it into `num_threads` chunks, sum
// each chunk on its own thread, then combine the partial sums.
//
// Two threading ideas come together here:
//   * A `move` closure is needed because each spawned thread must OWN the
//     `start`/`end` it works on (the thread may outlive this function's frame).
//   * `thread::spawn` returns a `JoinHandle<T>`; calling `.join()` blocks until
//     that thread finishes and hands back the value the closure returned — here,
//     the chunk's partial sum.
//
// TODO: Fill in the two marked spots so `parallel_sum` returns the real total.
//       Right now it spawns no threads and always returns 0.
//
// Run with `rustlings run parallel_sum1`, get a hint with `rustlings hint parallel_sum1`.

use std::thread;

// Sum 1..=n by dividing the range across `num_threads` worker threads.
// Assumes `num_threads >= 1` and `num_threads <= n`.
pub fn parallel_sum(n: u64, num_threads: u64) -> u64 {
    let chunk_size = n / num_threads;
    let mut handles: Vec<thread::JoinHandle<u64>> = Vec::new();

    for t in 0..num_threads {
        let start = t * chunk_size + 1;
        // The last thread takes whatever is left over so nothing is dropped
        // when `n` doesn't divide evenly by `num_threads`.
        let end = if t == num_threads - 1 {
            n
        } else {
            (t + 1) * chunk_size
        };

        // TODO: Spawn a thread with a `move` closure that computes the sum of
        // `start..=end` (e.g. `(start..=end).sum::<u64>()`) and returns it, then
        // push the returned `JoinHandle` into `handles`.
    }

    let mut total = 0;
    // TODO: Join every handle in `handles`. Each `handle.join().unwrap()`
    // returns that thread's partial sum — add it into `total`.

    total
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    // The closed form n*(n+1)/2 gives us a trustworthy expected value.
    fn gauss(n: u64) -> u64 {
        n * (n + 1) / 2
    }

    #[test]
    fn sum_divides_evenly() {
        assert_eq!(parallel_sum(100, 4), 5050);
        assert_eq!(parallel_sum(100, 4), gauss(100));
    }

    #[test]
    fn sum_with_leftover_chunk() {
        // 10 over 3 threads: chunks 1..=3, 4..=6, 7..=10 — the last absorbs the
        // remainder. 6 + 15 + 34 == 55.
        assert_eq!(parallel_sum(10, 3), 55);
        assert_eq!(parallel_sum(10, 3), gauss(10));
    }

    #[test]
    fn sum_large_range() {
        assert_eq!(parallel_sum(1_000_000, 8), gauss(1_000_000));
    }
}
