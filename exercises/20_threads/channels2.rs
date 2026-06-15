// channels2.rs
//
// The "multiple producer" in `mpsc` means you can `clone` the `Sender` and give
// each producer thread its own copy. The receiver can then be iterated with
// `for value in rx`, which yields every sent value and ends only when *all*
// senders have been dropped.
//
// This is also the classic gotcha: the original `tx` you created is still alive
// in the parent scope, so even after every cloned sender is dropped, the
// `for ... in rx` loop keeps waiting on that one remaining sender — forever.
// You must `drop(tx)` the original once you've finished cloning it.
// (NOTES "Message passing" / "The original tx never drops"; TRPL ch16.2)
//
// This mirrors the NOTES assignment of summing a large range across all cores:
// here we split `1..=100` into four chunks, sum each on its own thread, send the
// partial sums back, and add them up on the main thread.
//
// TODO: Give each producer its own sender by cloning `tx`, and drop the original
// `tx` before the receiving loop so the loop can ever finish.
//
// Run with `rustlings run channels2`, get a hint with
// `rustlings hint channels2`.

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
            // TODO: Each producer needs its OWN sending end so that the original
            // `tx` stays available to drop below. Clone `tx` into `tx_n`.
            let tx_n = ???;
            thread::spawn(move || {
                let partial: u32 = chunk.sum();
                tx_n.send(partial).unwrap();
            });
        }

        // TODO: Drop the original `tx`. Each cloned sender is dropped when its
        // thread finishes, but this one would stay alive and make the loop below
        // block forever waiting for a value that never comes.

        let mut total = 0;
        for partial in rx {
            total += partial;
        }

        // 1 + 2 + ... + 100 == 5050
        assert_eq!(total, 5050);
    }
}
