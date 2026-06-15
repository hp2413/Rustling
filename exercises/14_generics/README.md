# Generics

Generics is the topic of generalizing types and functionalities to broader cases.
This is extremely useful for reducing code duplication in many ways, but can call for some rather involved syntax.
Namely, being generic requires taking great care to specify over which types a generic type is actually considered valid.
The simplest and most common use of generics is for type parameters.

Generics show up in three places you'll practice here:

- **Generic functions** — `fn largest<T>(list: &[T]) -> T`.
- **Generic structs and their `impl` blocks** — a type parameter must be declared after `impl` before it can be used: `impl<T> Container<T>`.
- **Trait bounds** — a generic type can only use operations its bounds promise (e.g. `>` needs `PartialOrd`). A `where` clause keeps long bound lists readable.

## Further information

- [Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Trait Bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax)
- [Bounds](https://doc.rust-lang.org/rust-by-example/generics/bounds.html)
