//! Transposes

/// Transposes
pub trait TupleTranspose {
    type OutTuple;

    /// Transposes
    fn transpose(self) -> Self::OutTuple;
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
