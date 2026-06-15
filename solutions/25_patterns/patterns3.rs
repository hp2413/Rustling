// patterns3.rs
//
// Solution: `id @ 1..=5` binds `id` only when it falls in the range, and `..`
// ignores the `name` and `level` fields we don't need.

struct Player {
    id: u32,
    name: String,
    level: u32,
}

fn describe(player: Player) -> String {
    match player {
        Player { id: id @ 1..=5, .. } => format!("starter #{id}"),
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
