//! Map tuples to a single type
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

/// Map tuples to a single type
pub trait TupleUniformMapper<Input, Target> {
    type Output;

    /// Map tuples to a single type
    fn apply_uniform_map(self, input: Input) -> Self::Output;
}

/// Map tuples to a single type
pub trait TupleUniformMap<Target, Mapper: TupleUniformMapper<Self, Target>>: Sized {
    /// Map tuples to a single type
    fn uniform_map(self, mapper: Mapper) -> <Mapper as TupleUniformMapper<Self, Target>>::Output;
}

impl<Tuple, Target, Mapper: TupleUniformMapper<Tuple, Target>> TupleUniformMap<Target, Mapper> for Tuple {
    fn uniform_map(self, mapper: Mapper) -> <Mapper as TupleUniformMapper<Self, Target>>::Output {
        mapper.apply_uniform_map(self)
    }
}

impl<F: FnOnce(T) -> U, U, T> TupleUniformMapper<(T,), U> for F {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self)(input.0),)
    }
}

impl<F: FnOnce(&T) -> U, U, T> TupleUniformMapper<&(T,), U> for F {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self)(&input.0),)
    }
}

impl<F: FnOnce(&mut T) -> U, U, T> TupleUniformMapper<&mut (T,), U> for F {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self)(&mut input.0),)
    }
}

impl<F: FnOnce(T) -> U, U, T> TupleUniformMapper<(T,), U> for (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self.0)(input.0),)
    }
}

impl<F: FnMut(T) -> U, U, T> TupleUniformMapper<(T,), U> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self.0)(input.0),)
    }
}

impl<F: Fn(T) -> U, U, T> TupleUniformMapper<(T,), U> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: (T,)) -> Self::Output {
        ((self.0)(input.0),)
    }
}

impl<F: FnOnce(&T) -> U, U, T> TupleUniformMapper<&(T,), U> for (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self.0)(&input.0),)
    }
}

impl<F: FnMut(&T) -> U, U, T> TupleUniformMapper<&(T,), U> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self.0)(&input.0),)
    }
}

impl<F: Fn(&T) -> U, U, T> TupleUniformMapper<&(T,), U> for &(F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &(T,)) -> Self::Output {
        ((self.0)(&input.0),)
    }
}

impl<F: FnOnce(&mut T) -> U, U, T> TupleUniformMapper<&mut (T,), U> for (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self.0)(&mut input.0),)
    }
}

impl<F: FnMut(&mut T) -> U, U, T> TupleUniformMapper<&mut (T,), U> for &mut (F,) {
    type Output = (U,);

    fn apply_uniform_map(self, input: &mut (T,)) -> Self::Output {
        ((self.0)(&mut input.0),)
    }
}

impl<F: Fn(&mut T) -> U, U, T> TupleUniformMapper<&mut (T,), U> for &(F,) {
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
