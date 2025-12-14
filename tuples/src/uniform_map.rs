//! Map heterogeneous tuples to homogeneous tuples.
//!
//! ## Example
//!
//! -
//!   ```rust
//!   # use tuples::*;
//!   let a = (1, 2, 3);
//!   let r = a.uniform_map(|a| a > 0);
//!   assert_eq!(r, (true, true, true))
//!   ```
//! -
//!   ```rust
//!   # use tuples::*;
//!   let a = (1, 'c', true);
//!   let r = a.uniform_map((|a: i32| a > 0, |b: char| b.is_ascii(), |c: bool| c));
//!   assert_eq!(r, (true, true, true))
//!   ```
//! -
//!   ```rust
//!   # use tuples::*;
//!   let a = (1, 'c', true);
//!   let r = (&a).uniform_map((|a: &i32| *a > 0, |b: &char| b.is_ascii(), |c: &bool| *c));
//!   assert_eq!(r, (true, true, true))
//!   ```
//!

/// Map heterogeneous tuples to homogeneous tuples.
pub trait TupleUniformMapper<Input, Target> {
    type Output;

    /// Map heterogeneous tuples to homogeneous tuples.
    fn apply_uniform_map(self, input: Input) -> Self::Output;
}

/// Map heterogeneous tuples to homogeneous tuples.
pub trait TupleUniformMap<Target, Mapper: TupleUniformMapper<Self, Target>>: Sized {
    /// Map heterogeneous tuples to homogeneous tuples.
    fn uniform_map(self, mapper: Mapper) -> <Mapper as TupleUniformMapper<Self, Target>>::Output;
}

impl<Tuple, Target, Mapper: TupleUniformMapper<Tuple, Target>> TupleUniformMap<Target, Mapper> for Tuple {
    fn uniform_map(self, mapper: Mapper) -> <Mapper as TupleUniformMapper<Self, Target>>::Output {
        mapper.apply_uniform_map(self)
    }
}

impl<U, T, F: FnOnce(T) -> U> TupleUniformMapper<(T,), U> for F {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self)(input.0),)
    }
}

impl<U, T, F: FnOnce(&T) -> U> TupleUniformMapper<&(T,), U> for F {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self)(&input.0),)
    }
}

impl<U, T, F: FnOnce(&mut T) -> U> TupleUniformMapper<&mut (T,), U> for F {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self)(&mut input.0),)
    }
}

impl<U, T, F: FnOnce(T) -> U> TupleUniformMapper<(T,), U> for (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self.0)(input.0),)
    }
}

impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T,), U> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self.0)(input.0),)
    }
}

impl<U, T, F: Fn(T) -> U> TupleUniformMapper<(T,), U> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self.0)(input.0),)
    }
}

impl<U, T, F: FnOnce(&T) -> U> TupleUniformMapper<&(T,), U> for (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self.0)(&input.0),)
    }
}

impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T,), U> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self.0)(&input.0),)
    }
}

impl<U, T, F: Fn(&T) -> U> TupleUniformMapper<&(T,), U> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self.0)(&input.0),)
    }
}

impl<U, T, F: FnOnce(&mut T) -> U> TupleUniformMapper<&mut (T,), U> for (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self.0)(&mut input.0),)
    }
}

impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T,), U> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self.0)(&mut input.0),)
    }
}

impl<U, T, F: Fn(&mut T) -> U> TupleUniformMapper<&mut (T,), U> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self.0)(&mut input.0),)
    }
}

include!("./gen/uniform_map.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = (1, 2, 3);
        let r = a.uniform_map(|a| a > 0);
        assert_eq!(r, (true, true, true))
    }

    #[test]
    fn test1() {
        let a = (1, 'c', true);
        let r = a.uniform_map((|a: i32| a > 0, |b: char| b.is_ascii(), |c: bool| c));
        assert_eq!(r, (true, true, true))
    }

    #[test]
    fn test2() {
        let a = (1, 'c', true);
        let r = (&a).uniform_map((|a: &i32| *a > 0, |b: &char| b.is_ascii(), |c: &bool| *c));
        assert_eq!(r, (true, true, true))
    }
}
