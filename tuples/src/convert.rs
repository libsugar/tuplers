//! Traits for conversions between types

#![allow(unused_variables)]

use crate::{AnyTuple, Tuple, TupleItem};
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
pub trait AnyTupleAllInto<T>: AnyTuple {}

/// Mark tuple all item impled `From<T>`
pub trait AnyTupleAllFrom<T>: AnyTuple {}

/// Mark tuple all item impled `Into<T>`
pub trait TupleAllInto<T>: AnyTupleAllInto<T> + Tuple {
    type Item<const N: usize>: Into<T>
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<T>;

    fn item_into<const N: usize>(item: <Self as TupleAllInto<T>>::Item<N>) -> T
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<T>,
    {
        item.into()
    }
}

/// Mark tuple all item impled `From<T>`
pub trait TupleAllFrom<T>: AnyTupleAllFrom<T> + Tuple {
    type Item<const N: usize>: From<T>
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<T>;

    fn item_from<const N: usize>(val: T) -> <Self as TupleAllFrom<T>>::Item<N>
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<T>,
    {
        val.into()
    }
}

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
