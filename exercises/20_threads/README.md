# Threads

In most current operating systems, an executed program's code is run in a process, and the operating system manages multiple processes at once.
Within your program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads.

Because a spawned thread can outlive the code that created it, a closure given to `thread::spawn` cannot borrow local variables — use the `move` keyword to transfer ownership into the thread.

To communicate between threads, Rust favours *message passing* over shared state. An `mpsc` ("multiple producer, single consumer") channel gives you a `Sender` (`tx`) and a `Receiver` (`rx`): clone `tx` to hand a copy to each producer thread, then iterate the receiver with `for value in rx`. That loop ends only once every sender has been dropped — including the original `tx`, which you often need to `drop` explicitly.

## Further information

- [Dining Philosophers example](https://doc.rust-lang.org/1.4.0/book/dining-philosophers.html)
- [Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [Using `move` Closures with Threads](https://doc.rust-lang.org/book/ch16-01-threads.html#using-move-closures-with-threads)
- [Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)
