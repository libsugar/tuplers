//! Implemented iterators for tuples

use core::iter::{empty, once, Empty, FusedIterator, Once};
use core::mem::MaybeUninit;
use core::ops::Range;

pub trait TupleIter<'a> {
    type Iter: Iterator + 'a;

    fn iter(&'a self) -> Self::Iter;
}

pub trait TupleIntoIter {
    type Iter: Iterator;

    fn into_iter(self) -> Self::Iter;
}

pub trait TupleFromIter<T> {
    /// Like `Iter<T> -> (T, T, T)` with panic on failure
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
}

pub trait TupleTryFromIter<T>: Sized {
    /// Like `Iter<T> -> Option<(T, T, T)>`
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self>;
}

pub trait TupleFromIterTry<T> {
    type OutTuple;

    /// Like `Iter<T> -> (Option<T>, Option<T>, Option<T>)`
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple;
}

impl<'a> TupleIter<'a> for () {
    type Iter = Empty<()>;

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        empty()
    }
}

impl TupleIntoIter for () {
    type Iter = Empty<()>;

    #[inline]
    fn into_iter(self) -> Self::Iter {
        empty()
    }
}

impl<T> TupleFromIter<T> for () {
    fn from_iter<I: IntoIterator<Item = T>>(_: I) -> Self {
        ()
    }
}

impl<T> TupleTryFromIter<T> for () {
    fn try_from_iter<I: IntoIterator<Item = T>>(_: I) -> Option<Self> {
        Some(())
    }
}

impl<T> TupleFromIterTry<T> for () {
    type OutTuple = ();

    fn from_iter_try<I: IntoIterator<Item = T>>(_: I) -> Self {
        ()
    }
}

impl<'a, T: 'a> TupleIter<'a> for (T,) {
    type Iter = Once<&'a T>;

    #[inline]
    fn iter(&'a self) -> Self::Iter {
        once(&self.0)
    }
}

impl<T> TupleIntoIter for (T,) {
    type Iter = Once<T>;

    #[inline]
    fn into_iter(self) -> Self::Iter {
        once(self.0)
    }
}

impl<T> TupleFromIter<T> for (T,) {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        (iter.next().unwrap(),)
    }
}

impl<T> TupleTryFromIter<T> for (T,) {
    fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
        let mut iter = iter.into_iter();
        Some((iter.next()?,))
    }
}

impl<T> TupleFromIterTry<T> for (T,) {
    type OutTuple = (Option<T>,);
    fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
        let mut iter = iter.into_iter();
        (iter.next(),)
    }
}

include!(concat!(env!("OUT_DIR"), "/tuple_iter.rs"));

pub trait TupleCollect<T> {
    /// Like `Iter<T> -> (T, T, T)` with panic on failure
    fn collect_tuple<B: TupleFromIter<T>>(self) -> B;
    /// Like `Iter<T> -> Option<(T, T, T)>`
    fn try_collect_tuple<B: TupleTryFromIter<T>>(self) -> Option<B>;
    /// Like `Iter<T> -> (Option<T>, Option<T>, Option<T>)`
    fn collect_tuple_try<B: TupleFromIterTry<T>>(self) -> B::OutTuple;
}

impl<I: IntoIterator<Item = T>, T> TupleCollect<T> for I {
    fn collect_tuple<B: TupleFromIter<T>>(self) -> B {
        let iter = self.into_iter();
        B::from_iter(iter)
    }

    fn try_collect_tuple<B: TupleTryFromIter<T>>(self) -> Option<B> {
        let iter = self.into_iter();
        B::try_from_iter(iter)
    }

    fn collect_tuple_try<B: TupleFromIterTry<T>>(self) -> B::OutTuple {
        let iter = self.into_iter();
        B::from_iter_try(iter)
    }
}
