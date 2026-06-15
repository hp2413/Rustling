# Slices

A slice is a borrowed view into a contiguous sequence of elements — it stores a
pointer and a length, but never owns or copies the underlying data. You build
one with a range index where the end is exclusive: `&v[1..4]` borrows elements
at indices 1, 2 and 3.

- **Array / vector slices** have type `&[T]`. Both `[T; N]` arrays and `Vec<T>`
  coerce to `&[T]`, so a function that takes `&[T]` works on either.
- **String slices** have type `&str` and are a view into a `String` or string
  literal: `&s[0..5]`. String literals (`"hello"`) are themselves `&str`.

Preferring `&[T]` / `&str` parameters over `&Vec<T>` / `&String` makes functions
more general and avoids unnecessary allocation.

## Further information

- [The Book - The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust by Example - Arrays and Slices](https://doc.rust-lang.org/rust-by-example/primitives/array.html)
