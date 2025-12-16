#![allow(deprecated)]
//! Transposes
//!
//!  ```rust
//!  # use tuples::*;
//!  let a = Some((1, 2, 3)).transpose();
//!  assert_eq!(a, (Some(1), Some(2), Some(3)));
//!
//!  let b = (Some(1), Some(2), Some(3)).transpose();
//!  assert_eq!(b, Some((1, 2, 3)));
//!  ```
//!
//!  ```rust
//!  # use tuples::*;
//!  let a: (Result<u8, ()>, Result<u8, ()>, Result<u8, ()>) = (Ok(1), Ok(2), Ok(3));
//!  let b: Result<(u8, u8, u8), ()> = a.transpose();
//!  assert_eq!(b, Ok((1, 2, 3)));
//!  ```
//!
//!  ```rust
//!  # use tuples::*;
//!  let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
//!  let b: Result<(u8, u8, u8), i64> = a.transpose();
//!  assert_eq!(b, Err(-1));
//!  ```
//!
//!  ```rust
//!  # use tuples::*;
//!  let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
//!  let b = a.transpose::<i64>();
//!  assert_eq!(b, Err(-1));
//!  ```
//!
//!  ```rust
//!  # use tuples::*;
//!  let a: Result<(u8, u16, u32), (i32, i16, i8)> = Ok((1, 2, 3));
//!  let b: (Result<u8, i32>, Result<u16, i16>, Result<u32, i8>) = a.transpose();
//!  assert_eq!(b, (Ok(1), Ok(2), Ok(3)));
//!  ```

/// Transposes
pub trait TupleTranspose {
    type Output;

    /// Transposes
    fn transpose(self) -> Self::Output;
}

impl TupleTranspose for Option<()> {
    type Output = ();

    fn transpose(self) -> Self::Output {
        ()
    }
}

impl TupleTranspose for () {
    type Output = Option<()>;

    /// Always `None`
    fn transpose(self) -> Self::Output {
        None
    }
}

impl<T> TupleTranspose for Option<(T,)> {
    type Output = (Option<T>,);

    fn transpose(self) -> Self::Output {
        match self {
            Some(v) => (Some(v.0),),
            None => (None,),
        }
    }
}

impl<T> TupleTranspose for (Option<T>,) {
    type Output = Option<(T,)>;

    fn transpose(self) -> Self::Output {
        match self {
            (Some(v),) => Some((v,)),
            (None,) => None,
        }
    }
}

/// Transposes by into
pub trait TupleTransposeInto<U> {
    type ConvertedOutput;

    /// Transposes by into
    fn transpose_into(self) -> Self::ConvertedOutput;
}

/// Transposes by convert
pub trait TupleTransposeConvert {
    type Output<U>
    where
        Self: TupleTransposeInto<U>;

    /// Transposes by into
    fn transpose<U>(self) -> <Self as TupleTransposeConvert>::Output<U>
    where
        Self: TupleTransposeInto<U>;
}

include!("./gen/transpose.rs");

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        let a = Some((1, 2, 3)).transpose();
        assert_eq!(a, (Some(1), Some(2), Some(3)));
        let b = (Some(1), Some(2), Some(3)).transpose();
        assert_eq!(b, Some((1, 2, 3)));
    }

    #[test]
    fn test_result() {
        let a: (Result<u8, ()>, Result<u8, ()>, Result<u8, ()>) = (Ok(1), Ok(2), Ok(3));
        let b: Result<(u8, u8, u8), ()> = a.transpose();
        assert_eq!(b, Ok((1, 2, 3)));
    }

    #[test]
    fn test_result_1() {
        let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
        let b: Result<(u8, u8, u8), i64> = a.transpose();
        assert_eq!(b, Err(-1));
    }

    #[test]
    fn test_result_2() {
        let a: Result<(u8, u16, u32), (i32, i16, i8)> = Ok((1, 2, 3));
        let b: (Result<u8, i32>, Result<u16, i16>, Result<u32, i8>) = a.transpose();
        assert_eq!(b, (Ok(1), Ok(2), Ok(3)));
    }

    #[test]
    fn test_result_3() {
        let a: Result<((), (), ()), (u8, u8, u8)> = Err((1, 2, 3));
        let b: (Result<(), u8>, Result<(), u8>, Result<(), u8>) = a.transpose();
        assert_eq!(b, (Err(1), Err(2), Err(3)));
    }

    #[test]
    fn test_result_try() {
        fn f() -> Result<(), i64> {
            let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
            a.transpose::<i64>()?;
            Ok(())
        }
        let b = f();
        assert_eq!(b, Err(-1));
    }
}
