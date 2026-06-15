// channels1.rs
//
// Solution: `mpsc::channel()` gives a `(tx, rx)` pair; the producer thread
// sends the `String` with `tx.send(..).unwrap()`, and the main thread blocks on
// `rx.recv().unwrap()` to receive it.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn send_and_receive_one_message() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let message = String::from("Hello from the thread!");
            tx.send(message).unwrap();
        });

        let received = rx.recv().unwrap();

        assert_eq!(received, "Hello from the thread!");
    }
}
