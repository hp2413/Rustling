# Traits

A trait is a collection of methods.

Data types can implement traits. To do so, the methods making up the trait are defined for the data type. For example, the `String` data type implements the `From<&str>` trait. This allows a user to write `String::from("hello")`.

In this way, traits are somewhat similar to Java interfaces and C++ abstract classes.

Some additional common Rust traits include:

- `Clone` (the `clone` method)
- `Display` (which allows formatted display via `{}`)
- `Debug` (which allows formatted display via `{:?}`)

Because traits indicate shared behavior between data types, they are useful when writing generics.

Traits also support a few more advanced patterns:

- **Default implementations** — a trait method can carry a default body, which any implementing type inherits unless it overrides it.
- **Trait objects** (`Box<dyn Trait>`) — let you store values of different concrete types together and call them polymorphically through the trait at runtime (dynamic dispatch).
- **Operator overloading** — operators like `+` are sugar for trait methods (`std::ops::Add`), so implementing the trait makes the operator work on your own types.

## Further information

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [Default Implementations](https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations)
- [Using Trait Objects](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Operator Overloading](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html#default-generic-type-parameters-and-operator-overloading)
