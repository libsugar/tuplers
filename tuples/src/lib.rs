#![no_std]
#![allow(unused_imports)]

#[cfg(feature = "tuple_meta")]
mod meta {
    /// Tuple meta
    pub trait Tuple {
        /// Get arity of the tuple
        fn arity(&self) -> usize;
    }

    impl Tuple for () {
        fn arity(&self) -> usize {
            0
        }
    }

    impl<T> Tuple for (T,) {
        fn arity(&self) -> usize {
            1
        }
    }

    /// Mark traits for all tuples with all item is same type
    pub trait TupleSame<T>: Tuple {}

    impl<T> TupleSame<T> for () {}
    impl<T> TupleSame<T> for (T,) {}

    include!("./gen/tuple_impl.rs");
}
#[cfg(feature = "tuple_meta")]
pub use meta::*;

/// TupleN
#[cfg(feature = "tuple_meta")]
pub mod tuple_n {
    use crate::meta::*;

    pub trait Tuple0: Tuple {}
    impl Tuple0 for () {}

    pub trait Tuple1: Tuple {
        type Item0;
    }

    impl<T> Tuple1 for (T,) {
        type Item0 = T;
    }

    include!("./gen/tuple_n.rs");
}
#[cfg(all(feature = "tuple_meta", feature = "re-exports"))]
pub use tuple_n::*;

#[cfg(feature = "shorthand")]
mod shorthand {
    include!("./gen/tuple_alias.rs");

    /// Convenient shorthand
    ///
    /// # Syntax
    ///
    /// - `tuple![<type>; <repeat times>]`
    /// - `tuple![<expr>; <repeat times>]`
    /// - `tuple![<tuple size>; <type>, <type> ...]`
    ///
    /// # Examples
    /// *Repeat type*
    /// ```
    /// # use tuples::*;
    /// let a: tuple![u8; 3] = (5, 5, 5);
    /// # drop(a)
    /// ```
    /// *Repeat expr*
    /// ```
    /// # use tuples::*;
    /// let a: (u8, u8, u8) = tuple![5; 3];
    /// # drop(a)
    /// ```
    /// *Auto types*
    /// ```
    /// # use tuples::*;
    /// let a: tuple![3; u8] = (5, 5, 5usize);
    /// let b: (u8, i32, usize) = tuple![5; 3];
    /// assert_eq!(a, b);
    /// ```
    /// *Iter*
    /// ```
    /// # use tuples::*;
    /// let a = (1, 2, 3)
    ///     .into_iter()
    ///     .map(|v| v * 3)
    ///     .collect_tuple::<tuple![3;]>();
    /// let b: (i32, i32, i32) = (3, 6, 9);
    /// assert_eq!(a, b);
    /// ```
    /// ----
    /// ```ignore
    /// tuple![u8; 3] => (u8, u8, u8)
    /// tuple![5; 3] => (5, 5, 5)
    /// tuple![3; u8, u8, u8] => (u8, u8, u8)
    /// tuple![3; u8] => (u8, _, _)
    /// tuple![3;] => (_, _, _)
    /// ```
    #[macro_export]
    macro_rules! tuple {
        { $($t:tt)* } => { tuple_! { $($t)* } }
    }
}
#[cfg(feature = "shorthand")]
pub use shorthand::*;

#[cfg(feature = "tuple_as")]
pub mod tuple_as;
#[cfg(all(feature = "tuple_as", feature = "re-exports"))]
pub use tuple_as::*;

#[cfg(feature = "tuple_iter")]
pub mod tuple_iter;
#[cfg(all(feature = "tuple_iter", feature = "re-exports"))]
pub use tuple_iter::*;

#[cfg(feature = "tuple_map")]
pub mod tuple_map;
#[cfg(all(feature = "tuple_map", feature = "re-exports"))]
pub use tuple_map::*;

#[cfg(feature = "combin")]
pub mod combin;
#[cfg(all(feature = "combin", feature = "re-exports"))]
pub use combin::*;

/// Split tuples
#[cfg(any(feature = "split_by", feature = "split_to_tuple_by", feature = "split_at", feature = "split_to_tuple_at"))]
pub mod split {
    #[cfg(feature = "split_parts")]
    pub mod split_parts;
    #[cfg(all(feature = "split_parts", feature = "re-exports"))]
    pub use split_parts::*;

    #[cfg(feature = "split_by")]
    pub mod split_by;
    #[cfg(all(feature = "split_by", feature = "re-exports"))]
    pub use split_by::*;

    #[cfg(feature = "split_to_tuple_by")]
    pub mod split_to_tuple_by;
    #[cfg(all(feature = "split_to_tuple_by", feature = "re-exports"))]
    pub use split_to_tuple_by::*;

    #[cfg(feature = "split_at")]
    pub mod split_at;
    #[cfg(all(feature = "split_at", feature = "re-exports"))]
    pub use split_at::*;

    #[cfg(feature = "split_to_tuple_at")]
    pub mod split_to_tuple_at;
    #[cfg(all(feature = "split_to_tuple_at", feature = "re-exports"))]
    pub use split_to_tuple_at::*;
}
#[cfg(all(any(feature = "split_by", feature = "split_to_tuple_by", feature = "split_at", feature = "split_to_tuple_at"), feature = "re-exports"))]
pub use split::*;

#[cfg(feature = "transpose")]
pub mod transpose;
#[cfg(all(feature = "transpose", feature = "re-exports"))]
pub use transpose::*;

#[cfg(feature = "flatten")]
pub mod flatten;
#[cfg(all(feature = "flatten", feature = "re-exports"))]
pub use flatten::*;

#[cfg(feature = "cloned")]
pub mod cloned;
#[cfg(all(feature = "cloned", feature = "re-exports"))]
pub use cloned::*;

#[cfg(feature = "tuple_call")]
pub mod tuple_call;
#[cfg(all(feature = "tuple_call", feature = "re-exports"))]
pub use tuple_call::*;

#[cfg(feature = "apply_tuple")]
pub mod apply_tuple;
#[cfg(all(feature = "apply_tuple", feature = "re-exports"))]
pub use apply_tuple::*;

#[cfg(feature = "capt")]
pub mod capt;
#[cfg(all(feature = "capt", feature = "re-exports"))]
pub use capt::*;

#[cfg(feature = "tuple_get")]
pub mod tuple_get;
#[cfg(all(feature = "tuple_get", feature = "re-exports"))]
pub use tuple_get::*;

#[cfg(all(feature = "sort", feature = "tuple_meta", feature = "tuple_get"))]
pub mod sort;
#[cfg(all(feature = "sort", feature = "tuple_meta", feature = "tuple_get", feature = "re-exports"))]
pub use sort::*;

#[cfg(any(feature = "tuple_swap_n", test, doc))]
pub mod tuple_swap_n;
#[cfg(any(all(feature = "tuple_swap_n", feature = "re-exports"), test, doc))]
pub use tuple_swap_n::*;

#[cfg(any(feature = "permutations", test, doc))]
pub mod permutations;
#[cfg(any(all(feature = "permutations", feature = "re-exports"), test, doc))]
pub use permutations::*;

#[cfg(any(feature = "combinations", test, doc))]
pub mod combinations;
#[cfg(any(all(feature = "combinations", feature = "re-exports"), test, doc))]
pub use combinations::*;

#[cfg(any(feature = "afn", test, doc))]
pub mod afn;

#[cfg(any(feature = "uniform_map", test, doc))]
pub mod uniform_map;
#[cfg(any(all(feature = "uniform_map", feature = "re-exports"), test, doc))]
pub use uniform_map::*;
