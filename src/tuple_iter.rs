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
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self;
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

include!(concat!(env!("OUT_DIR"), "/tuple_iter.rs"));

pub trait TupleCollect<T> {
    fn collect_tuple<B: TupleFromIter<T>>(self) -> B;
}

impl<I: IntoIterator<Item = T>, T> TupleCollect<T> for I {
    fn collect_tuple<B: TupleFromIter<T>>(self) -> B {
        let iter = self.into_iter();
        B::from_iter(iter)
    }
}
