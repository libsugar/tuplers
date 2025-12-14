//! Traits for conversions between types

use crate::AnyTuple;
use core::{
    convert::Infallible,
    ops::{Deref, DerefMut},
};

/// AsRef for Tuple
pub trait TupleAsRef<'a> {
    type Output: 'a;

    /// AsRef for Tuple
    fn as_ref(&'a self) -> Self::Output;
}

/// AsMut for Tuple
pub trait TupleAsMut<'a> {
    type Output: 'a;

    /// AsMut for Tuple
    fn as_mut(&'a mut self) -> Self::Output;
}

/// AsDeref for Tuple
pub trait TupleAsDeref<'a> {
    type Output: 'a;

    /// AsDeref for Tuple
    fn as_deref(&'a self) -> Self::Output;
}

/// AsDerefMut for Tuple
pub trait TupleAsDerefMut<'a> {
    type Output: 'a;

    /// AsDerefMut for Tuple
    fn as_deref_mut(&'a mut self) -> Self::Output;
}

/// Mapping item to `Option` for Tuple
pub trait TupleAsOption {
    type Output;

    /// Mapping item to `Option::Some` for Tuple
    fn as_some(self) -> Self::Output;
}

/// Mapping item to `Result` for Tuple
pub trait TupleAsResultOk<E> {
    type Output;

    /// Mapping item to `Result::Ok` for Tuple
    fn as_ok(self) -> Self::Output;
}

/// Mapping item to `Result` for Tuple
pub trait TupleAsResultErr<T> {
    type Output;

    /// Mapping item to `Result::Err` for Tuple
    fn as_err(self) -> Self::Output;
}

/// Mark tuple all item impled `Into<T>`
pub trait TupleAllInto<T> {}

/// Mark tuple all item impled `From<T>`
pub trait TupleAllFrom<T> {}

/// Into for tuple
pub trait TupleInto<T> {
    /// Into for tuple
    fn tuple_into(self) -> T;
}

/// From for tuple
pub trait TupleFrom<T> {
    /// From for tuple
    fn tuple_from(src: T) -> Self;
}

/// TryInto for tuple
pub trait TupleTryInto<T>: Sized {
    type Output;

    /// TryInto for tuple
    fn tuple_try_into(self) -> Self::Output;
}

/// TryFrom for tuple
pub trait TupleTryFrom<T>: Sized {
    type Output;

    /// TryFrom for tuple
    fn tuple_try_from(src: T) -> Self::Output;
}

include!("./gen/convert.rs");

impl TupleInto<()> for () {
    fn tuple_into(self: ()) -> () {
        self
    }
}

impl TupleFrom<()> for () {
    fn tuple_from(src: ()) -> () {
        src
    }
}

impl TupleTryInto<()> for () {
    type Output = ();

    fn tuple_try_into(self) -> () {
        self
    }
}

impl TupleTryFrom<()> for () {
    type Output = ();

    fn tuple_try_from(src: ()) -> () {
        src
    }
}
