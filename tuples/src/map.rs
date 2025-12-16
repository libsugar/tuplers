//! Mapping tuple items
//!
//! ## Examples
//!
//! - map nth item
//!    ```rust
//!    # use tuples::*;
//!    let a = (1i32, 2f32, 3f64, 4u8, 5usize);
//!    let b = a.map::<3>(|v| v * 5u8);
//!    assert_eq!(b, (1i32, 2f32, 3f64, 20u8, 5usize));
//!    ```
//! - map all for homogeneous tuples
//!    ```rust
//!    # use tuples::*;
//!    let a = (1, 2, 3);
//!    let b = a.map_all(|v| v * 3);
//!    assert_eq!(b, (3, 6, 9));
//!    ```
//! - map all for heterogeneous tuples
//!    ```rust
//!    # use tuples::*;
//!    let a = (1i32, 2f32, 3f64);
//!    let b = a.map_all((|v| v * 10i32, |v| v * 100f32, |v| v * 1000f64));
//!    assert_eq!(b, (10i32, 200f32, 3000f64));
//!    ```
//!
//! - dynamic map
//!    ```rust
//!    # use tuples::*;
//!    let a = (1, 2, 3);
//!    let r: Result<(i32, i32, i32), (i32, i32, i32)> = a.dyn_map(1, |v| v * 3);
//!    let b = r.unwrap();
//!    assert_eq!(b, (1, 6, 3));
//!    ```

use crate::{AnyHomogeneousTuple, AnyTuple, HomogeneousTuple, Tuple, TupleItem, misc::RefOrMut};

/// Dynamic mapping
pub trait TupleDynamicMap<T>: AnyHomogeneousTuple<T> + Sized {
    /// Return `Ok` if n is less than the arity of the tuple  
    /// `()` always return `Err`  
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self>;
}

impl<T> TupleDynamicMap<T> for () {
    fn dyn_map(self, _n: usize, _mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        Err(())
    }
}

////////////////////

pub use mapper::*;
/// Mapper
pub mod mapper {
    use super::*;

    /// A mapper that can map the item `N` in a tuple `T`
    pub trait TupleMapperOnceN<const N: usize, T: Tuple + TupleItem<N>> {
        type OutputN;

        /// Mapping the term `N` of tuple `T`
        fn do_map_once_n(self, item: T::Item<N>) -> Self::OutputN;
    }

    impl<const N: usize, T: Tuple + TupleItem<N>, U, F: FnOnce(<T as Tuple>::Item<N>) -> U> TupleMapperOnceN<N, T> for F {
        type OutputN = U;

        fn do_map_once_n(self, item: <T as Tuple>::Item<N>) -> Self::OutputN {
            (self)(item)
        }
    }

    impl<const N: usize, T: Tuple + TupleItem<N>, U, F: FnOnce(<T as Tuple>::Item<N>) -> U> TupleMapperOnceN<N, T> for (F,) {
        type OutputN = U;

        fn do_map_once_n(self, item: <T as Tuple>::Item<N>) -> Self::OutputN {
            (self.0)(item)
        }
    }

    /// A mapper that can map the item `N` in a tuple `T`
    pub trait TupleMapperMutN<const N: usize, T: Tuple + TupleItem<N>>: TupleMapperOnceN<N, T> {
        /// Mapping the term `N` of tuple `T`
        fn do_map_mut_n(&mut self, item: T::Item<N>) -> Self::OutputN;
    }

    impl<const N: usize, T: Tuple + TupleItem<N>, U, F: FnMut(<T as Tuple>::Item<N>) -> U> TupleMapperMutN<N, T> for F {
        fn do_map_mut_n(&mut self, item: <T as Tuple>::Item<N>) -> Self::OutputN {
            (self)(item)
        }
    }

    impl<const N: usize, T: Tuple + TupleItem<N>, U, F: FnMut(<T as Tuple>::Item<N>) -> U> TupleMapperMutN<N, T> for (F,) {
        fn do_map_mut_n(&mut self, item: <T as Tuple>::Item<N>) -> Self::OutputN {
            (self.0)(item)
        }
    }

    /// A mapper that can map the item `N` in a tuple `T`
    pub trait TupleMapperN<const N: usize, T: Tuple + TupleItem<N>>: TupleMapperMutN<N, T> {
        /// Mapping the term `N` of tuple `T`
        fn do_map_n(&self, item: T::Item<N>) -> Self::OutputN;
    }

    impl<const N: usize, T: Tuple + TupleItem<N>, U, F: Fn(<T as Tuple>::Item<N>) -> U> TupleMapperN<N, T> for F {
        fn do_map_n(&self, item: <T as Tuple>::Item<N>) -> Self::OutputN {
            (self)(item)
        }
    }

    impl<const N: usize, T: Tuple + TupleItem<N>, U, F: Fn(<T as Tuple>::Item<N>) -> U> TupleMapperN<N, T> for (F,) {
        fn do_map_n(&self, item: <T as Tuple>::Item<N>) -> Self::OutputN {
            (self.0)(item)
        }
    }

    ////////////////////

    /// A mapper that can map the tuple `T`
    pub trait TupleMapperOnce<T: Tuple> {
        /// The type after mapping of item `N` of tuple `T`
        type Output<const N: usize>
        where
            T: TupleItem<N>,
            Self: TupleMapperOnceN<N, T>;

        /// Mapping the term `N` of tuple `T`
        fn do_map_once<const N: usize>(self, item: T::Item<N>) -> Self::Output<N>
        where
            T: TupleItem<N>,
            Self: TupleMapperOnceN<N, T>;
    }

    impl<T: Tuple, M> TupleMapperOnce<T> for M {
        type Output<const N: usize>
            = <Self as TupleMapperOnceN<N, T>>::OutputN
        where
            T: TupleItem<N>,
            Self: TupleMapperOnceN<N, T>;

        fn do_map_once<const N: usize>(self, item: <T as Tuple>::Item<N>) -> Self::Output<N>
        where
            T: TupleItem<N>,
            Self: TupleMapperOnceN<N, T>,
        {
            self.do_map_once_n(item)
        }
    }

    /// A mapper that can map the tuple `T`
    pub trait TupleMapperMut<T: Tuple>: TupleMapperOnce<T> {
        /// Mapping the term `N` of tuple `T`
        fn do_map_mut<const N: usize>(&mut self, item: T::Item<N>) -> Self::Output<N>
        where
            T: TupleItem<N>,
            Self: TupleMapperMutN<N, T>;
    }

    impl<T: Tuple, M> TupleMapperMut<T> for M {
        fn do_map_mut<const N: usize>(&mut self, item: T::Item<N>) -> Self::Output<N>
        where
            T: TupleItem<N>,
            Self: TupleMapperMutN<N, T>,
        {
            self.do_map_mut_n(item)
        }
    }

    /// A mapper that can map the tuple `T`
    pub trait TupleMapper<T: Tuple>: TupleMapperMut<T> {
        /// Mapping the term `N` of tuple `T`
        fn do_map<const N: usize>(&self, item: T::Item<N>) -> Self::Output<N>
        where
            T: TupleItem<N>,
            Self: TupleMapperN<N, T>;
    }

    impl<T: Tuple, M> TupleMapper<T> for M {
        fn do_map<const N: usize>(&self, item: T::Item<N>) -> Self::Output<N>
        where
            T: TupleItem<N>,
            Self: TupleMapperN<N, T>,
        {
            self.do_map_n(item)
        }
    }
}

////////////////////

/// Mapping nth item of tuple
pub trait TupleMapN<const N: usize, Mapper>: Tuple + Sized {
    /// The type returned after item mapping
    type OutputN;

    /// Mapping nth item of tuple
    fn map_n(self, mapper: Mapper) -> Self::OutputN;
}

/// Mapping tuple items
pub trait TupleMap<Mapper>: Tuple + Sized {
    /// The type returned after item mapping
    type Output<const N: usize>
    where
        Self: TupleMapN<N, Mapper>;

    /// Mapping nth item of tuple
    fn map<const N: usize>(self, mapper: Mapper) -> Self::Output<N>
    where
        Self: TupleMapN<N, Mapper>;
}

impl<T: Tuple + Sized, M> TupleMap<M> for T {
    type Output<const N: usize>
        = <Self as TupleMapN<N, M>>::OutputN
    where
        Self: TupleMapN<N, M>;

    fn map<const N: usize>(self, mapper: M) -> Self::Output<N>
    where
        Self: TupleMapN<N, M>,
    {
        self.map_n(mapper)
    }
}

////////////////////

/// Mapping nth item of tuple with arg
pub trait TupleMapWithN<A, const N: usize, Mapper>: Tuple + Sized {
    /// The type returned after item mapping with arg
    type OutputN;

    /// Mapping nth item of tuple with arg
    fn map_with_n(self, arg: A, mapper: Mapper) -> Self::OutputN;
}

/// Mapping tuple items with arg
pub trait TupleMapWith<A, Mapper>: Tuple + Sized {
    /// The type returned after item mapping with arg
    type Output<const N: usize>
    where
        Self: TupleMapWithN<A, N, Mapper>;

    /// Mapping nth item of tuple with arg
    fn map_with<const N: usize>(self, arg: A, mapper: Mapper) -> Self::Output<N>
    where
        Self: TupleMapWithN<A, N, Mapper>;
}

impl<A, T: Tuple + Sized, M> TupleMapWith<A, M> for T {
    type Output<const N: usize>
        = <Self as TupleMapWithN<A, N, M>>::OutputN
    where
        Self: TupleMapWithN<A, N, M>;

    fn map_with<const N: usize>(self, arg: A, mapper: M) -> Self::Output<N>
    where
        Self: TupleMapWithN<A, N, M>,
    {
        self.map_with_n(arg, mapper)
    }
}

////////////////////

/// Mapping all items of the tuple
pub trait TupleMapAll<Mapper>: Tuple + Sized {
    type Output;

    /// Mapping all items of the tuple
    fn map_all(self, mapper: Mapper) -> Self::Output;
}

impl<M> TupleMapAll<M> for () {
    type Output = ();

    fn map_all(self, _mapper: M) -> () {
        self
    }
}

/// Mapping all items of the tuple with arg
pub trait TupleMapAllWith<A, Mapper>: Tuple + Sized {
    type Output;

    /// Mapping all items of the tuple with arg
    fn map_all_with(self, arg: A, mapper: Mapper) -> Self::Output;
}

/// Mapping all items of the tuple with arg
pub trait TupleMapAllWithMut<'a, A, Mapper>: Tuple + Sized {
    type Output;

    /// Mapping all items of the tuple with arg
    fn map_all_with_mut(self, arg: &'a mut A, mapper: Mapper) -> Self::Output;
}

impl<A, M> TupleMapAllWith<A, M> for () {
    type Output = ();

    fn map_all_with(self, _arg: A, _mapper: M) -> Self::Output {
        self
    }
}

impl<'a, A, M> TupleMapAllWithMut<'a, A, M> for () {
    type Output = ();

    fn map_all_with_mut(self, _arg: &'a mut A, _mapper: M) -> Self::Output {
        self
    }
}

////////////////////

pub use marker::*;
/// Marker
pub mod marker {
    use super::*;

    /// Mark the mapping output is homogeneous tuple
    pub trait TupleMapIntoHomogeneous<T, Mapper>: TupleMapAll<Mapper>
    where
        Self::Output: HomogeneousTuple<T>,
    {
    }

    /// Mark the mapping input is homogeneous tuple
    pub trait TupleMapFromHomogeneous<T, Mapper>: TupleMapAll<Mapper> + HomogeneousTuple<T> {}

    impl<T, Mapper, S: TupleMapAll<Mapper>> TupleMapIntoHomogeneous<T, Mapper> for S where Self::Output: HomogeneousTuple<T> {}

    impl<T, Mapper, S: TupleMapAll<Mapper>> TupleMapFromHomogeneous<T, Mapper> for S where Self: HomogeneousTuple<T> {}
}

////////////////////

impl<T0, M> TupleMapAll<M> for (T0,)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type Output = (M::Output<0>,);

    fn map_all(self, mapper: M) -> Self::Output {
        (mapper.do_map_once(self.0),)
    }
}

impl<A, T, U, M> TupleMapAllWith<A, M> for (T,)
where
    M: FnOnce(A, T) -> U,
{
    type Output = (U,);

    fn map_all_with(self, arg: A, mapper: M) -> Self::Output {
        ((mapper)(arg, self.0),)
    }
}

impl<'a, A: 'a, T, U, M> TupleMapAllWithMut<'a, A, M> for (T,)
where
    M: FnOnce(&'a mut A, T) -> U,
{
    type Output = (U,);

    fn map_all_with_mut(self, arg: &'a mut A, mapper: M) -> Self::Output {
        ((mapper)(arg, self.0),)
    }
}

// todo gen

include!("./gen/map.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let f = |a: i32| a as f32 + 1.5;
        let t = (1, 2);
        let r = t.map_all(f);
        assert_eq!(r, (2.5, 3.5))
    }

    #[test]
    fn test2() {
        let f = |a: i32| a as f32 + 1.5;
        let t = (1, 2);
        let r = t.map::<0>(f);
        assert_eq!(r, (2.5, 2))
    }

    #[test]
    fn test_dyn() {
        let a = (1, 2, 3);
        let r: Result<(i32, i32, i32), (i32, i32, i32)> = a.dyn_map(1, |v| v * 3);
        let b = r.unwrap();
        assert_eq!(b, (1, 6, 3));
    }

    #[test]
    fn test() {
        let a = (1, 2, 3);
        let b = a.map_all(|v| v * 3);
        assert_eq!(b, (3, 6, 9));
    }

    #[test]
    fn test_n() {
        let a = (1i32, 2f32, 3f64, 4u8, 5usize);
        let b = a.map::<3>(|v| v * 5u8);
        assert_eq!(b, (1i32, 2f32, 3f64, 20u8, 5usize));
    }

    #[test]
    fn test_all() {
        let a = (1i32, 2f32, 3f64);
        let b = a.map_all((|v| v * 10i32, |v| v * 100f32, |v| v * 1000f64));
        assert_eq!(b, (10i32, 200f32, 3000f64));
    }

    #[test]
    fn test_all_with_1() {
        let a = (1, 2);
        let b = a.map_all_with(&3, |a: &i32, b| *a * b);
        assert_eq!(b, (3, 6));
    }

    #[test]
    fn test_all_with_2() {
        let a = (1, 2);
        let b = a.map_all_with_mut(&mut 3, |a: &mut i32, b| *a * b);
        assert_eq!(b, (3, 6));
    }

    #[test]
    fn test_n_with() {
        let a = (1, 2, 3);
        let b = a.map_with::<1>(&3, |a: &i32, v| v * *a);
        assert_eq!(b, (1, 6, 3));
    }
}
