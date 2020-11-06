#![no_std]

/// AsRef for Tuple
pub trait TupleAsRef<'a> {
    type OutTuple: 'a;

    /// AsRef for Tuple
    fn as_ref(&'a self) -> Self::OutTuple;
}

impl<'a, T: 'a> TupleAsRef<'a> for (T,) {
    type OutTuple = (&'a T,);

    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0,)
    }
}

/// AsMut for Tuple
pub trait TupleAsMut<'a> {
    type OutTuple: 'a;

    /// AsMut for Tuple
    fn as_mut(&'a mut self) -> Self::OutTuple;
}

impl<'a, T: 'a> TupleAsMut<'a> for (T,) {
    type OutTuple = (&'a mut T,);

    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0,)
    }
}

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

/// TupleN
pub mod tuple_n {
    use crate::*;

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

include!(concat!(env!("OUT_DIR"), "/tuple_alias.rs"));

#[cfg(feature = "re-exports")]
pub use tuple_n::*;

#[cfg(feature = "tuple_iter")]
pub mod tuple_iter;
#[cfg(all(feature = "tuple_iter", feature = "re-exports"))]
pub use tuple_iter::*;

#[cfg(feature = "tuple_map")]
pub mod tuple_map;
#[cfg(all(feature = "tuple_map", feature = "re-exports"))]
pub use tuple_map::*;
