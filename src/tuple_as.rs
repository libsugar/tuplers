//! As X

use core::ops::{Deref, DerefMut};

include!(concat!(env!("OUT_DIR"), "/tuple_as.rs"));

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

/// Mapping item to `Option` for Tuple
pub trait TupleAsOption {
    type OutTuple;

    /// Mapping item to `Option::Some` for Tuple
    fn as_some(self) -> Self::OutTuple;
}

impl<T> TupleAsOption for (T,) {
    type OutTuple = (Option<T>,);

    fn as_some(self) -> Self::OutTuple {
        (Some(self.0),)
    }
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

impl<T, E> TupleAsResultOk<E> for (T,) {
    type OutTuple = (Result<T, E>,);

    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0),)
    }
}

impl<T, O> TupleAsResultErr<O> for (T,) {
    type OutTuple = (Result<O, T>,);

    fn as_err(self) -> Self::OutTuple {
        (Err(self.0),)
    }
}

/// AsDeref for Tuple
pub trait TupleAsDeref<'a> {
    type OutTuple: 'a;

    /// AsDeref for Tuple
    fn as_deref(&'a self) -> Self::OutTuple;
}

impl<'a, T: 'a + Deref> TupleAsDeref<'a> for (T,) {
    type OutTuple = (&'a <T as Deref>::Target,);

    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(),)
    }
}

/// AsDerefMut for Tuple
pub trait TupleAsDerefMut<'a> {
    type OutTuple: 'a;

    /// AsDerefMut for Tuple
    fn as_deref_mut(&'a mut self) -> Self::OutTuple;
}

impl<'a, T: 'a + DerefMut> TupleAsDerefMut<'a> for (T,) {
    type OutTuple = (&'a mut <T as Deref>::Target,);

    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(),)
    }
}
