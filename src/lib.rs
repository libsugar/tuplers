#![no_std]

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

    include!(concat!(env!("OUT_DIR"), "/tuple_impl.rs"));
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

    include!(concat!(env!("OUT_DIR"), "/tuple_n.rs"));
}
#[cfg(all(feature = "tuple_meta", feature = "re-exports"))]
pub use tuple_n::*;

include!(concat!(env!("OUT_DIR"), "/tuple_alias.rs"));

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

#[cfg(feature = "transpose")]
pub mod transpose;
#[cfg(all(feature = "transpose", feature = "re-exports"))]
pub use transpose::*;
