//! Map heterogeneous tuples to homogeneous tuples by arg
//!
//! ## Example
//!
//! -
//!   ```rust
//!   # use tuples::*;
//!   let a = (1, 2, 3);
//!   let r = a.uniform_map_by(&0, |x: &i32, a: i32| a > *x);
//!   assert_eq!(r, (true, true, true))
//!   ```
//! -
//!   ```rust
//!   # use tuples::*;
//!   let a = (1, 'c');
//!   let r = a.uniform_map_by(&0, (|x: &i32, a: i32| a > *x, |x: &i32, b: char| b as i32 > *x));
//!   assert_eq!(r, (true, true))
//!   ```
//! -
//!   ```rust
//!   # use tuples::*;
//!   let a = (1, 'c');
//!   let r = a.uniform_map_by(&mut 0, (|x: &mut i32, a: i32| a > *x, |x: &mut i32, b: char| b as i32 > *x));
//!   assert_eq!(r, (true, true))
//!   ```
//!

use crate::param::Param;

/// Map heterogeneous tuples to homogeneous tuples by arg
pub trait TupleUniformMapperBy<Input, Target, Arg> {
    type Output;

    /// Map heterogeneous tuples to homogeneous tuples by arg
    fn apply_uniform_map_by(self, arg: Arg, input: Input) -> Self::Output;
}

/// Map heterogeneous tuples to homogeneous tuples by arg
pub trait TupleUniformMapBy<Target, Arg, Mapper: TupleUniformMapperBy<Self, Target, Arg>>: Sized {
    /// Map heterogeneous tuples to homogeneous tuples by arg
    fn uniform_map_by(self, arg: Arg, mapper: Mapper) -> <Mapper as TupleUniformMapperBy<Self, Target, Arg>>::Output;
}

impl<Tuple, Target, Arg, Mapper: TupleUniformMapperBy<Tuple, Target, Arg>> TupleUniformMapBy<Target, Arg, Mapper> for Tuple {
    fn uniform_map_by(self, arg: Arg, mapper: Mapper) -> <Mapper as TupleUniformMapperBy<Self, Target, Arg>>::Output {
        mapper.apply_uniform_map_by(arg, self)
    }
}

impl<A, T, U, F: FnOnce(A, T) -> U> TupleUniformMapperBy<(T,), U, A> for F {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: (T,)) -> Self::Output {
        ((self)(arg, input.0),)
    }
}

impl<A, T, U, F: FnOnce(A, &T) -> U> TupleUniformMapperBy<&(T,), U, A> for F {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &(T,)) -> Self::Output {
        ((self)(arg, &input.0),)
    }
}

impl<A, T, U, F: FnOnce(A, &mut T) -> U> TupleUniformMapperBy<&mut (T,), U, A> for F {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &mut (T,)) -> Self::Output {
        ((self)(arg, &mut input.0),)
    }
}

impl<A, T, U, F: FnOnce(A, T) -> U> TupleUniformMapperBy<(T,), U, A> for (F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: (T,)) -> Self::Output {
        ((self.0)(arg, input.0),)
    }
}

impl<A, T, U, F: FnMut(A, T) -> U> TupleUniformMapperBy<(T,), U, A> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: (T,)) -> Self::Output {
        ((self.0)(arg, input.0),)
    }
}

impl<A, T, U, F: Fn(A, T) -> U> TupleUniformMapperBy<(T,), U, A> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: (T,)) -> Self::Output {
        ((self.0)(arg, input.0),)
    }
}

impl<A, T, U, F: FnOnce(A, &T) -> U> TupleUniformMapperBy<&(T,), U, A> for (F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &(T,)) -> Self::Output {
        ((self.0)(arg, &input.0),)
    }
}

impl<A, T, U, F: FnMut(A, &T) -> U> TupleUniformMapperBy<&(T,), U, A> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &(T,)) -> Self::Output {
        ((self.0)(arg, &input.0),)
    }
}

impl<A, T, U, F: Fn(A, &T) -> U> TupleUniformMapperBy<&(T,), U, A> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &(T,)) -> Self::Output {
        ((self.0)(arg, &input.0),)
    }
}

impl<A, T, U, F: FnOnce(A, &mut T) -> U> TupleUniformMapperBy<&mut (T,), U, A> for (F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &mut (T,)) -> Self::Output {
        ((self.0)(arg, &mut input.0),)
    }
}

impl<A, T, U, F: FnMut(A, &mut T) -> U> TupleUniformMapperBy<&mut (T,), U, A> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &mut (T,)) -> Self::Output {
        ((self.0)(arg, &mut input.0),)
    }
}

impl<A, T, U, F: Fn(A, &mut T) -> U> TupleUniformMapperBy<&mut (T,), U, A> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map_by(self, arg: A, input: &mut (T,)) -> Self::Output {
        ((self.0)(arg, &mut input.0),)
    }
}

include!("./gen/uniform_map_by.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = (1, 2, 3);
        let r = a.uniform_map_by(&0, |x: &i32, a: i32| a > *x);
        assert_eq!(r, (true, true, true))
    }

    #[test]
    fn test1() {
        let a = (1, 'c');
        let r = a.uniform_map_by(&0, (|x: &i32, a: i32| a > *x, |x: &i32, b: char| b as i32 > *x));
        assert_eq!(r, (true, true))
    }

    #[test]
    fn test2() {
        let a = (1, 'c');
        let r = a.uniform_map_by(&mut 0, (|x: &mut i32, a: i32| a > *x, |x: &mut i32, b: char| b as i32 > *x));
        assert_eq!(r, (true, true))
    }
}
