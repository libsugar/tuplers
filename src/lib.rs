
/// AsRef for Tuple
pub trait TupleAsRef<'a> {
    type OutTuple: 'a;

    /// AsRef for Tuple
    fn as_ref(&'a self) -> Self::OutTuple;
}

/// AsMut for Tuple
pub trait TupleAsMut<'a> {
    type OutTuple: 'a;

    /// AsMut for Tuple
    fn as_mut(&'a mut self) -> Self::OutTuple;
}

/// Tuple meta
pub trait Tuple {
    /// Tuple size
    fn size(&self) -> usize;
}

/// Mark traits for all tuples with all item is same type
pub trait TupleSame<T>: Tuple { }

include!(concat!(env!("OUT_DIR"), "/tuple_impl.rs"));

/// TupleN
pub mod tuple_n {
    use crate::*;
    
    include!(concat!(env!("OUT_DIR"), "/tuple_n.rs"));
}
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