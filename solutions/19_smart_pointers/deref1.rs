// deref1.rs
//
// Solution: a `MyBox<T>` wraps a single `T`, so dereferencing it should produce
// that `T`. Set `type Target = T` and return a reference to the tuple-struct
// field, `&self.0`.

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
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
        assert_eq!(*b, 5);
    }

    #[test]
    fn deref_coercion_to_str() {
        let name = MyBox::new(String::from("Ferris"));
        assert_eq!(hello(&name), "Hello, Ferris!");
    }

    fn hello(name: &str) -> String {
        format!("Hello, {name}!")
    }
}
