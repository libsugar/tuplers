#![allow(deprecated)]
//! Transposes

/// Transposes
pub trait TupleTranspose {
    type OutTuple;

    /// Transposes
    fn transpose(self) -> Self::OutTuple;
}

impl TupleTranspose for Option<()> {
    type OutTuple = ();

    fn transpose(self) -> Self::OutTuple {
        ()
    }
}

impl TupleTranspose for () {
    type OutTuple = Option<()>;

    /// Always `None`
    fn transpose(self) -> Self::OutTuple {
        None
    }
}

impl<T> TupleTranspose for Option<(T,)> {
    type OutTuple = (Option<T>,);

    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0),),
            None => (None,),
        }
    }
}

impl<T> TupleTranspose for (Option<T>,) {
    type OutTuple = Option<(T,)>;

    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v),) => Some((v,)),
            (None,) => None,
        }
    }
}

/// Transposes for Result
pub trait TupleTransposeResult<Eo> {
    type OutTuple;

    /// Transposes for Result
    #[deprecated = "use transpose1"]
    fn transpose(self) -> Self::OutTuple;
}

impl<T, Eo: From<E>, E> TupleTransposeResult<Eo> for (Result<T, E>,) {
    type OutTuple = Result<(T,), Eo>;

    fn transpose(self) -> Self::OutTuple {
        let (v0,) = self;
        Ok((v0?,))
    }
}

/// Transposes for Result
pub trait TupleTransposeResultSameError {
    type OutTuple;

    /// Transposes for Result
    fn transpose_same_error(self) -> Self::OutTuple;
}

impl<T, E> TupleTransposeResultSameError for (Result<T, E>,) {
    type OutTuple = Result<(T,), E>;

    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0,) = self;
        Ok((v0?,))
    }
}

/// Transposes for Result
pub trait TupleTransposeResult1_1<E> {
    type OutTuple<Eo>;

    /// Transposes for Result
    fn transpose1<Eo: From<E>>(self) -> Self::OutTuple<Eo>;
}

impl<T, E> TupleTransposeResult1_1<E> for (Result<T, E>,) {
    type OutTuple<Eo> = Result<(T,), Eo>;

    fn transpose1<Eo: From<E>>(self) -> Self::OutTuple<Eo> {
        let (v0,) = self;
        Ok((v0?,))
    }
}

/// Transposes for Result
pub trait TupleTransposeResultMapErr1<E> {
    type OutTuple<Eo>;

    /// Transposes for Result
    fn transpose_map_err<Eo>(self, f: impl FnOnce(E) -> Eo) -> Self::OutTuple<Eo>;
}

impl<T, E> TupleTransposeResultMapErr1<E> for (Result<T, E>,) {
    type OutTuple<Eo> = Result<(T,), Eo>;

    fn transpose_map_err<Eo>(self, f: impl FnOnce(E) -> Eo) -> Self::OutTuple<Eo> {
        let (v0,) = self;
        Ok((match v0 {
            Ok(v) => v,
            Err(e) => Err(f(e))?,
        },))
    }
}

include!("./gen/transpose.rs");

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
fn test_result_2() {
    let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
    let b: Result<(u8, u8, u8), i64> = a.transpose();
    assert_eq!(b, Err(-1));
}

#[test]
fn test_result_same_error() {
    let a: (Result<u8, i64>, Result<u8, i64>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
    let b: Result<(u8, u8, u8), i64> = a.transpose_same_error();
    assert_eq!(b, Err(-1));
}

#[test]
fn test_result_same_error_2() {
    let a: (Result<u8, ()>, Result<u8, ()>, Result<u8, ()>) = (Ok(1), Ok(2), Ok(3));
    let b: Result<(u8, u8, u8), ()> = a.transpose_same_error();
    assert_eq!(b, Ok((1, 2, 3)));
}

#[test]
fn test_result_gat() {
    let a: (Result<u8, ()>, Result<u8, ()>, Result<u8, ()>) = (Ok(1), Ok(2), Ok(3));
    let b: Result<(u8, u8, u8), ()> = a.transpose1();
    assert_eq!(b, Ok((1, 2, 3)));
}

#[test]
fn test_result_gat_2() {
    let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
    let b: Result<(u8, u8, u8), i64> = a.transpose1();
    assert_eq!(b, Err(-1));
}

#[test]
fn test_result_gat_try() {
    fn f() -> Result<(), i64> {
        let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
        a.transpose1::<i64>()?;
        Ok(())
    }
    let b = f();
    assert_eq!(b, Err(-1));
}

#[test]
fn test_result_map_err() {
    let a: (Result<u8, ()>, Result<u8, ()>, Result<u8, ()>) = (Ok(1), Ok(2), Ok(3));
    let b: Result<(u8, u8, u8), ()> = a.transpose_map_err(|a| a, |a| a, |a| a);
    assert_eq!(b, Ok((1, 2, 3)));
}

#[test]
fn test_result_map_err_2() {
    let a: (Result<u8, i16>, Result<u8, i32>, Result<u8, i64>) = (Ok(1), Err(-1), Ok(3));
    let b: Result<(u8, u8, u8), i64> = a.transpose_map_err(|a| a.into(), |a| a.into(), |a| a.into());
    assert_eq!(b, Err(-1));
}
