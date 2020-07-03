
# Few 
## A generalization of `std::Option` allowing for up to two optional values.

This library provides a `Few` enum with three variants:

```rust
pub enum Few<T> {
    Zero,
    One(T),
    Two(T, T),
}
```

Very few methods are defined for it, and for most purposes, [`std::Option`](https://doc.rust-lang.org/stable/std/option/enum.Option.html), [`std::Vec`](https://doc.rust-lang.org/stable/std/vec/struct.Vec.html), or [`smallvec`](https://crates.io/crates/smallvec) should be used instead. This library was developed to provide a data structure for pattern matching on the result of set-like `intersect`, `union`, and `minus` operations over contiguous ranges.

# License

Few is licenced with the [MIT license](/license-mit.md) or the [Apache version 2.0 license](/license-apache.md), at your option.

