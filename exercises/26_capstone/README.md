# Capstone

These two exercises pull together concepts from earlier modules into small,
realistic programs — without any command-line or file I/O, so everything is
driven by tests.

- **`word_count1`** rebuilds the search core of the classic `minigrep` project
  from TRPL ch12. It practises **string slices**, **iterators** (`.lines()`),
  and **lifetimes**: the returned `Vec<&str>` borrows into the searched text, so
  the `'a` lifetime ties them together.
- **`parallel_sum1`** is the "sum `1..=N` using all the cores" exercise. It
  divides a range into chunks, sums each chunk on its own thread with a `move`
  closure, and combines the partial sums returned by each `JoinHandle`.

## Further information

- [An I/O Project: Building a Command Line Program (minigrep)](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)
- [Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [The String Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
