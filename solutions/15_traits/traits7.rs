// traits7.rs
//
// Solution: annotate the `Vec` element type as the trait object
// `Box<dyn Animal>` and box each value so the differing concrete types can live
// in one `Vec` and be called polymorphically.

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
        let animals: Vec<Box<dyn Animal>> = vec![Box::new(Sheep), Box::new(Cow)];

        assert_eq!(all_noises(&animals), vec!["baaah", "moooo"]);
    }
}
