// channels1.rs
//
// `mpsc` stands for "multiple producer, single consumer". A channel has two
// ends: a `Sender` (`tx`) you can clone and move into producer threads, and a
// single `Receiver` (`rx`) that collects the values. `tx.send(v)` returns a
// `Result` (it fails if the receiver was dropped), and `rx.recv()` blocks until
// a value arrives, also returning a `Result`. (TRPL ch16.2 "Message Passing")
//
// TODO: Build a channel, send a `String` from a spawned thread, and receive it
// on the main thread. Replace each `???` so the test compiles and passes.
//
// Run with `rustlings run channels1`, get a hint with
// `rustlings hint channels1`.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn send_and_receive_one_message() {
        // TODO: Create a channel with `mpsc::channel()`. It returns a
        // `(Sender, Receiver)` tuple — destructure it into `tx` and `rx`.
        let (tx, rx) = ???;

        // The sending end is moved into a new thread: the producer.
        thread::spawn(move || {
            let message = String::from("Hello from the thread!");
            // TODO: Send `message` down the channel. `send` returns a `Result`,
            // so call `.unwrap()` on it.
            ???;
        });

        // TODO: Receive the value on the main thread (the consumer). `recv()`
        // blocks until a message arrives and returns a `Result` — unwrap it.
        let received = ???;

        assert_eq!(received, "Hello from the thread!");
    }
}
