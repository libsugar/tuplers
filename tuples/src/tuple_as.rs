//! As X

use core::ops::{Deref, DerefMut};

include!("./gen/tuple_as.rs");

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

/// Mapping item to `Option` for Tuple
pub trait TupleAsOption {
    type OutTuple;

    /// Mapping item to `Option::Some` for Tuple
    fn as_some(self) -> Self::OutTuple;
}

/// Mapping item to `Result` for Tuple
pub trait TupleAsResultOk<E> {
    type OutTuple;

    /// Mapping item to `Result::Ok` for Tuple
    fn as_ok(self) -> Self::OutTuple;
}

/// Mapping item to `Result` for Tuple
pub trait TupleAsResultErr<T> {
    type OutTuple;

    /// Mapping item to `Result::Err` for Tuple
    fn as_err(self) -> Self::OutTuple;
}

/// AsDeref for Tuple
pub trait TupleAsDeref<'a> {
    type OutTuple: 'a;

    /// AsDeref for Tuple
    fn as_deref(&'a self) -> Self::OutTuple;
}

/// AsDerefMut for Tuple
pub trait TupleAsDerefMut<'a> {
    type OutTuple: 'a;

    /// AsDerefMut for Tuple
    fn as_deref_mut(&'a mut self) -> Self::OutTuple;
}
