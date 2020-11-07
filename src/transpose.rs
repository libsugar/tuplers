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

include!(concat!(env!("OUT_DIR"), "/transpose.rs"));

#[test]
fn test() {
    let a = Some((1, 2, 3)).transpose();
    assert_eq!(a, (Some(1), Some(2), Some(3)));
    let b = (Some(1), Some(2), Some(3)).transpose();
    assert_eq!(b, Some((1, 2, 3)));
}
