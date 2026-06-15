// parallel_sum1.rs
//
// Solution: spawn one `move` thread per chunk, each returning its partial sum,
// then join them all and add the partials together.

use std::thread;

pub fn parallel_sum(n: u64, num_threads: u64) -> u64 {
    let chunk_size = n / num_threads;
    let mut handles: Vec<thread::JoinHandle<u64>> = Vec::new();

    for t in 0..num_threads {
        let start = t * chunk_size + 1;
        let end = if t == num_threads - 1 {
            n
        } else {
            (t + 1) * chunk_size
        };

        let handle = thread::spawn(move || (start..=end).sum::<u64>());
        handles.push(handle);
    }

    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }

    total
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(parallel_sum(10, 3), 55);
        assert_eq!(parallel_sum(10, 3), gauss(10));
    }

    #[test]
    fn sum_large_range() {
        assert_eq!(parallel_sum(1_000_000, 8), gauss(1_000_000));
    }
}
