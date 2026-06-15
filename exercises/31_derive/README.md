# Derivable Traits

Some traits are so common — and so mechanical to implement — that the compiler
can generate the `impl` for you. You opt in with the `#[derive(...)]` attribute
placed directly above a `struct` or `enum`.

Commonly derived traits:

- **`Debug`** — formatting with `{:?}` / `{:#?}`.
- **`Clone`** / **`Copy`** — explicit `.clone()`, and implicit copy-on-assign.
- **`PartialEq`** / **`Eq`** — comparison with `==` / `!=`.
- **`PartialOrd`** / **`Ord`** — ordering, sorting, `min` / `max`.
- **`Hash`** — using a value as a `HashMap` / `HashSet` key (together with `Eq`).
- **`Default`** — a zero-argument `T::default()` constructor.

A type can only derive a trait if all of its fields already implement it.
Equality and ordering are computed field-by-field in declaration order.

## Further information

- [Derivable Traits (Appendix C)](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [Rust by Example - derive](https://doc.rust-lang.org/rust-by-example/trait/derive.html)
