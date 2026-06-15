# Closures

Closures are anonymous functions you can save in a variable or pass as
arguments to other functions. Unlike functions, closures can *capture* values
from the scope in which they're defined.

A closure captures its environment in one of three ways, which map directly to
the three closure traits:

- **`Fn`** — borrows captured values immutably (`&T`).
- **`FnMut`** — borrows captured values mutably (`&mut T`); it can change them.
- **`FnOnce`** — takes ownership of captured values; it can be called only once.

The compiler infers the least restrictive trait automatically. Adding the
`move` keyword forces a closure to take ownership of everything it captures —
which is what you need when the closure must outlive the current scope, for
example when it is moved into another thread.

## Further information

- [Closures: Anonymous Functions that Capture Their Environment](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [Rust by Example - Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
