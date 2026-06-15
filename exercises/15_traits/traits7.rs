// traits7.rs
//
// A `Vec<T>` can only hold ONE concrete type, so you can't put a `Sheep` and a
// `Cow` in the same `Vec` directly. Trait objects fix this: store each value
// behind a pointer such as `Box<dyn Animal>`. Now the element type is "some
// value that implements `Animal`", and the right `noise` method is chosen at
// RUNTIME (dynamic dispatch). (TRPL ch17.2 "Using Trait Objects")
//
// TODO: Fix the `Vec` in the test so it can hold both a `Sheep` and a `Cow` as
// trait objects.
//
// Run with `rustlings run traits7`, get a hint with `rustlings hint traits7`.

trait Animal {
    fn noise(&self) -> String;
}

struct Sheep;
struct Cow;

impl Animal for Sheep {
    fn noise(&self) -> String {
        String::from("baaah")
    }
}

impl Animal for Cow {
    fn noise(&self) -> String {
        String::from("moooo")
    }
}

// Collect the noise every animal makes. The animals have different concrete
// types, so this only works because they arrive as trait objects behind a
// `Box<dyn Animal>` and we call `noise` through the `Animal` trait.
fn all_noises(animals: &[Box<dyn Animal>]) -> Vec<String> {
    animals.iter().map(|animal| animal.noise()).collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_animals_in_one_vec() {
        // TODO: This `Vec` must hold a `Sheep` AND a `Cow`. Give it the element
        // type `Box<dyn Animal>` and box each value, e.g. `Box::new(Sheep)`.
        let animals: Vec<???> = vec![Sheep, Cow];

        assert_eq!(all_noises(&animals), vec!["baaah", "moooo"]);
    }
}
