// deref1.rs
//
// The dereference operator `*` is not built into your own types — it is provided
// by the `Deref` trait. When you write `*value`, the compiler actually runs
// `*(value.deref())`. By implementing `Deref` you make a custom smart pointer
// behave like a reference: `*my_box` yields the value inside. It also enables
// "deref coercion", where a `&MyBox<String>` can be passed to a function that
// wants a `&str`, because the compiler keeps calling `deref` until the types
// line up. (TRPL ch15.2)
//
// `MyBox<T>` is a one-field tuple struct that owns a `T`. Right now
// `*MyBox::new(5)` won't compile, because `MyBox` doesn't implement `Deref`.
//
// TODO: Complete the `Deref` implementation for `MyBox<T>`: set the associated
// `Target` type and return a reference to the wrapped value from `deref`.
//
// Run with `rustlings run deref1`, get a hint with `rustlings hint deref1`.

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // TODO: Dereferencing a `MyBox<T>` should produce a `T`. Set the target.
    type Target = ???;

    fn deref(&self) -> &Self::Target {
        // TODO: Return a reference to the single field of this tuple struct.
        ???
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_returns_inner_value() {
        let b = MyBox::new(5);
        // `*b` only works once `MyBox` implements `Deref`.
        assert_eq!(*b, 5);
    }

    #[test]
    fn deref_coercion_to_str() {
        // `MyBox<String>` derefs to `String`, and `String` derefs to `str`, so a
        // `&MyBox<String>` coerces all the way down to the `&str` that `hello`
        // expects.
        let name = MyBox::new(String::from("Ferris"));
        assert_eq!(hello(&name), "Hello, Ferris!");
    }

    fn hello(name: &str) -> String {
        format!("Hello, {name}!")
    }
}
