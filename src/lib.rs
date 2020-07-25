////////////////////////////////////////////////////////////////////////////////
// Few -- A generalization of `std::Option` allowing for up to two optional
// values.
////////////////////////////////////////////////////////////////////////////////
// Copyright 2020 Skylor R. Schermer
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! A generalization of `std::Option` allowing for up to two optional values.
//! 
//! This library provides a `Few` enum with three variants:
//! 
//! ```rust
//! pub enum Few<T> {
//!     Zero,
//!     One(T),
//!     Two(T, T),
//! }
//! ```
//! 
//! Very few methods are defined for it, and for most purposes, [`std::Option`],
//! [`std::Vec`], or [`smallvec`] should be used instead. This library was
//! developed to provide a data structure for pattern matching on the result of
//! set-like `intersect`, `union`, and `minus` operations over contiguous
//! ranges.
//!
//! # Features
//!
//! | Feature | Description |
//! | ------- | ----------- |
//! | "serde" | Enables serialization and deserialization of data using [serde](https://crates.io/crates/serde). |
//!
//! By default, there are no features enabled.
//!
//! [`std::Option`]: https://doc.rust-lang.org/stable/std/option/enum.Option.html
//! [`std::Vec`]: https://doc.rust-lang.org/stable/std/vec/struct.Vec.html
//! [`smallvec`]: https://crates.io/crates/smallvec
////////////////////////////////////////////////////////////////////////////////
#![warn(anonymous_parameters)]
#![warn(bad_style)]
#![warn(bare_trait_objects)]
#![warn(const_err)]
#![warn(dead_code)]
#![warn(elided_lifetimes_in_paths)]
#![warn(improper_ctypes)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_doc_code_examples)]
#![warn(missing_docs)]
#![warn(no_mangle_generic_items)]
#![warn(non_shorthand_field_patterns)]
#![warn(nonstandard_style)]
#![warn(overflowing_literals)]
#![warn(path_statements)]
#![warn(patterns_in_fns_without_body)]
#![warn(private_in_public)]
#![warn(rust_2018_idioms)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unconditional_recursion)]
#![warn(unreachable_pub)]
#![warn(unused)]
#![warn(unused_allocation)]
#![warn(unused_comparisons)]
#![warn(unused_parens)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![warn(while_true)]

// External library imports.
#[cfg(feature = "serde")]
use serde::{ Serialize, Deserialize };


////////////////////////////////////////////////////////////////////////////////
// Few
////////////////////////////////////////////////////////////////////////////////
/// A type which may contain zero, one, or two of a value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Few<T> {
    /// No value present.
    Zero,
    /// One value present.
    One(T),
    /// Two values present.
    Two(T, T),
}

impl<T> Few<T> {
    /// Returns true if the `Few` is a `Zero` value.
    pub fn is_zero(&self) -> bool {
        match self {
            Few::Zero => true,
            _         => false,
        }
    }

    /// Returns true if the `Few` is a `One` value.
    pub fn is_one(&self) -> bool {
        match self {
            Few::One(_) => true,
            _           => false,
        }
    }

    /// Returns true if the `Few` is a `Two` value.
    pub fn is_two(&self) -> bool {
        match self {
            Few::Two(_, _) => true,
            _              => false,
        }
    }

    /// Returns true if the `Few` is a `One` or `Two` value containing the given
    /// value.
    pub fn contains<U>(&self, x: &U) -> bool
        where U: PartialEq<T>
    {
        match self {
            Few::Zero      => false,
            Few::One(v)    => x == v,
            Few::Two(a, b) => x == a || x == b,
        }
    }

    /// Maps an Few<T> to Few<U> by applying a function to a contained value.
    pub fn map<F, U>(self, mut f: F) -> Few<U>
        where F: FnMut(T) -> U,
    {
        match self {
            Few::Zero      => Few::Zero,
            Few::One(v)    => Few::One((f)(v)),
            Few::Two(a, b) => Few::Two((f)(a), (f)(b)),
        }
    }
}

impl<T> Iterator for Few<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let mut res = None;
        replace_with(self, |curr|
            match curr {
                Few::Zero      => { res = None;    Few::Zero },
                Few::One(v)    => { res = Some(v); Few::Zero },
                Few::Two(a, b) => { res = Some(a); Few::One(b) },
            }
        );
        res
    }
}

impl<T> DoubleEndedIterator for Few<T> {
    fn next_back(&mut self) -> Option<T> {
        let mut res = None;
        replace_with(self, |curr|
            match curr {
                Few::Zero      => { res = None;    Few::Zero },
                Few::One(v)    => { res = Some(v); Few::Zero },
                Few::Two(a, b) => { res = Some(b); Few::One(a) },
            }
        );
        res
    }
}

impl<T> ExactSizeIterator for Few<T> {
    fn len(&self) -> usize {
        match self {
            Few::Zero      => 0,
            Few::One(_)    => 1,
            Few::Two(_, _) => 2,
        }
    }
}

impl<T> std::iter::FusedIterator for Few<T> {}


impl<T> Default for Few<T> {
    fn default() -> Self {
        Few::Zero
    }
}

impl<T> From<T> for Few<T> {
    fn from(value: T) -> Self {
        Few::One(value)
    }
}

impl<T> From<(T, T)> for Few<T> {
    fn from(value: (T, T)) -> Self {
        Few::Two(value.0, value.1)
    }
}

impl<T> From<Option<T>> for Few<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            None        => Few::Zero,
            Some(value) => Few::One(value),
        }
    }
}

impl<T> From<Option<(T, T)>> for Few<T> {
    fn from(value: Option<(T, T)>) -> Self {
        match value {
            None         => Few::Zero,
            Some((a, b)) => Few::Two(a, b),
        }
    }
}

impl<T> From<(Option<T>, Option<T>)> for Few<T> {
    fn from(value: (Option<T>, Option<T>)) -> Self {
        match (value.0, value.1) {
            (None,    None)    => Few::Zero,
            (Some(a), None)    => Few::One(a),
            (None,    Some(b)) => Few::One(b),
            (Some(a), Some(b)) => Few::Two(a, b),
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// replace_with
////////////////////////////////////////////////////////////////////////////////
/// Replaces the value behind a mut reference with the result of a closure
/// called on the value. Will abort if a panic occurs in the given closure.
#[inline]
fn replace_with<T, F>(val: &mut T, replace: F)
    where F: FnOnce(T) -> T {
    let guard = ExitGuard;

    unsafe {
        let old = std::ptr::read(val);
        let new = replace(old);
        std::ptr::write(val, new);
    }

    std::mem::forget(guard);
}

struct ExitGuard;

impl Drop for ExitGuard {
    fn drop(&mut self) {
        panic!("`replace_with` closure unwind");
    }
}
