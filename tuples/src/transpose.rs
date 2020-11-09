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
    fn transpose(self) -> Self::OutTuple;
}

impl<T, Eo: From<E>, E> TupleTransposeResult<Eo> for (Result<T, E>,) {
    type OutTuple = Result<(T,), Eo>;

    fn transpose(self) -> Self::OutTuple {
        let (v0,) = self;
        Ok((v0?,))
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
