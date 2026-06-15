# Flow control

Rust has three looping constructs:

- **`loop`** repeats forever until you `break`. It is also an expression — a
  `break value;` makes that value the result of the whole loop.
- **`while <condition>`** repeats while the condition is `true`. Its cousin
  **`while let <pattern> = <expr>`** repeats as long as the value keeps matching
  the pattern (great for draining an iterator or a stack with `.pop()`).
- **`for x in iterable`** walks an iterator or a range. Ranges can be exclusive
  (`0..n`) or inclusive (`1..=n`).

When loops are nested, a plain `break`/`continue` only affects the innermost
loop. Add a **loop label** (`'outer:`) and target it (`break 'outer;`) to break
or continue an enclosing loop.

## Further information

- [The Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [The Book - Loop labels](https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops)
- [Rust by Example - Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)
