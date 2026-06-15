# Patterns and Matching

Patterns are how Rust destructures and inspects values. A `match` expression
compares a value against a series of patterns and runs the first arm that fits;
`match` must be *exhaustive*, so every possible value has to be covered.

Patterns can be refined and combined in many ways:

- **Guards** — an extra `if` condition on an arm: `n if n < 0 => ...`.
- **Ranges** — match a span of values: `1..=9 => ...`.
- **Destructuring** — pull fields out of structs, tuples, and enums:
  `Point { x, y }`, `Some((a, b))`, and nested combinations of these.
- **`@` bindings** — bind a value *and* test it: `id @ 1..=5`.
- **`_` and `..`** — ignore a single value, or ignore the rest of the
  fields/elements.
- **`if let` / `while let` / `let else`** — ergonomic forms for when you only
  care about a single pattern.

## Further information

- [Pattern syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
- [Rust by Example - match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [Rust by Example - if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [Rust by Example - while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
