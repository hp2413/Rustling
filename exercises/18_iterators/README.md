# Iterators

This section will teach you about Iterators.

Iterators turn collections into a sequence of values you can transform with
*adapters* (`map`, `filter`, `zip`, `enumerate`, `scan`, ...) and finish with
*consumers* (`sum`, `product`, `fold`, `partition`, `collect`). How you create
the iterator matters too: `iter()` borrows (`&T`), `iter_mut()` borrows mutably
(`&mut T`), and `into_iter()` takes ownership (`T`).

You can also make your OWN type iterable by implementing the `Iterator` trait:
define `type Item` and `fn next(&mut self) -> Option<Self::Item>`, and every
adapter and consumer above becomes available for free.

## Further information

- [Iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [Creating Our Own Iterators with the Iterator Trait](https://doc.rust-lang.org/book/ch13-02-iterators.html#creating-our-own-iterators-with-the-iterator-trait)
- [Iterator documentation](https://doc.rust-lang.org/stable/std/iter/)
- [Iterator method index](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
