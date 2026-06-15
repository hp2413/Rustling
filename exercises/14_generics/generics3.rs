// generics3.rs
//
// A struct can be generic over the type it stores, and its methods can be
// generic too. The key rule: a type parameter must be DECLARED right after
// `impl` before you can use it on the type. So an impl block for `Container<T>`
// has to read `impl<T> Container<T>` — the first `<T>` introduces the
// parameter, the second uses it. (TRPL ch10.1 "Generic Data Types")
//
// TODO: Make the `impl` block below generic so `Container` works for ANY type.
//
// Run with `rustlings run generics3`, get a hint with `rustlings hint generics3`.

struct Container<T> {
    item: T,
}

// TODO: This impl block won't compile because it uses `T` without declaring it.
// Declare the parameter right after `impl`, and keep using it on the type:
//   impl<T> Container<T> { ... }
impl Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

    // A generic method: it returns a reference to the stored value, whatever
    // its type happens to be.
    fn get(&self) -> &T {
        &self.item
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn holds_an_integer() {
        let container = Container::new(42);
        assert_eq!(*container.get(), 42);
    }

    #[test]
    fn holds_a_string() {
        let container = Container::new(String::from("Ferris"));
        assert_eq!(*container.get(), String::from("Ferris"));
    }
}
