// channels2.rs
//
// Solution: clone `tx` once per producer thread, then `drop(tx)` the original.
// Once every cloned sender has been dropped (each thread drops its own when it
// finishes), the `for partial in rx` loop ends and we sum the partials.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn parallel_partial_sums() {
        let (tx, rx) = mpsc::channel();

        let chunks = [1..=25, 26..=50, 51..=75, 76..=100];
        for chunk in chunks {
            let tx_n = tx.clone();
            thread::spawn(move || {
                let partial: u32 = chunk.sum();
                tx_n.send(partial).unwrap();
            });
        }

        drop(tx);

        let mut total = 0;
        for partial in rx {
            total += partial;
        }

        assert_eq!(total, 5050);
    }
}
