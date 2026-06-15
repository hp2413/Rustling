# Smart Pointers

In Rust, smart pointers are variables that contain an address in memory and reference some other data, but they also have additional metadata and capabilities.
Smart pointers in Rust often own the data they point to, while references only borrow data.

Beyond owning heap data, the standard smart pointers customize behavior through
traits: `Deref` makes a type act like a reference (`*value`), `Drop` runs
cleanup code automatically when a value goes out of scope (RAII), and
`RefCell<T>` enforces the borrowing rules at runtime to give you *interior
mutability* — often combined as `Rc<RefCell<T>>` for shared, mutable data.

Be careful when `Rc`s point at each other: a strong cycle never frees and leaks
memory. `Weak<T>` (made with `Rc::downgrade`) is a non-owning reference that
breaks such cycles — access it with `.upgrade()`, which yields `None` once the
target has been dropped.

## Further Information

- [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [Treating Smart Pointers Like Regular References with Deref](https://doc.rust-lang.org/book/ch15-02-deref.html)
- [Running Code on Cleanup with the Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)
- [Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [RefCell\<T\> and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [Reference Cycles Can Leak Memory (Weak\<T\>)](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
- [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [Cow Documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
