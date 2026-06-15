// patterns3.rs
//
// Two more pattern tools:
//   * an `@` binding lets you *test* a value against a pattern AND *keep* it in
//     a variable at the same time: `id @ 1..=5` matches ids 1 through 5 and
//     binds the actual value to `id`;
//   * `..` ignores the fields (or tuple/array elements) you don't care about,
//     so you don't have to name every field of a struct.
// (TRPL "Pattern syntax".)
//
// TODO: Complete the first match arm so it matches only players whose `id` is
// in the range 1..=5, binds that id, and ignores the other fields.
//
// Run with `rustlings run patterns3`, get a hint with `rustlings hint patterns3`.

struct Player {
    id: u32,
    name: String,
    level: u32,
}

fn describe(player: Player) -> String {
    match player {
        // TODO: Replace `???` with a pattern that matches only ids 1..=5 using
        // an `@` binding (`id @ 1..=5`) and ignores the remaining fields with
        // `..`. The second arm below shows how `..` is used.
        ??? => format!("starter #{id}"),
        Player { id, .. } => format!("veteran #{id}"),
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn player(id: u32) -> Player {
        Player {
            id,
            name: String::from("anon"),
            level: 1,
        }
    }

    #[test]
    fn describes_players() {
        assert_eq!(describe(player(1)), "starter #1");
        assert_eq!(describe(player(5)), "starter #5");
        assert_eq!(describe(player(6)), "veteran #6");
        assert_eq!(describe(player(99)), "veteran #99");
    }
}
