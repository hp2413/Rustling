# Formatting

Rust's printing macros — `print!`, `println!`, `eprintln!`, and the
string-building `format!` — share a small formatting mini-language inside the
`{}` placeholders. You can select arguments by position (`{0}`) or by name
(`{name}`), and add a format spec after a colon to control precision (`{:.2}`),
width and alignment (`{:>8}`), fill characters, sign, and number base.

Two traits decide how a value is rendered:

- **`Display`** (`{}`) — the user-facing form. You implement it yourself; it is
  never derived. Implementing it also gives you `.to_string()` for free.
- **`Debug`** (`{:?}`, pretty `{:#?}`) — the programmer-facing form. Almost
  always obtained with `#[derive(Debug)]`.

## Further information

- [Rust by Example - Formatted print](https://doc.rust-lang.org/rust-by-example/hello/print.html)
- [Rust by Example - Display](https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html)
- [The `std::fmt` module](https://doc.rust-lang.org/std/fmt/index.html)
- [The Book - Derived Traits (`Debug`)](https://doc.rust-lang.org/book/ch05-02-an-example-program-using-structs.html#adding-useful-functionality-with-derived-traits)
