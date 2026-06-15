// generics3.rs
//
// Solution: declare the type parameter after `impl` (`impl<T>`) and use it on
// the type (`Container<T>`). Now `new` and the generic method `get` work for
// any stored type.

struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }

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
