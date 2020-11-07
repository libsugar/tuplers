//! Combine tuples

/// Add Item on Left
pub trait CombinLeft<T> {
    type Out;

    /// Add Item on Left
    fn left(self, target: T) -> Self::Out;
}

/// Add Item on Right
pub trait CombinRight<T> {
    type Out;

    /// Add Item on Right
    fn push(self, target: T) -> Self::Out;
}

/// Concat Tuples  
/// `(a, b).concat((c, d)) -> (a, b, c, d)`
pub trait CombinConcat<T> {
    type Out;

    /// Concat Tuples  
    /// `(a, b).concat((c, d)) -> (a, b, c, d)`
    fn concat(self, target: T) -> Self::Out;
}

impl<T> CombinLeft<T> for () {
    type Out = (T,);

    fn left(self, target: T) -> Self::Out {
        (target,)
    }
}

impl<T> CombinRight<T> for () {
    type Out = (T,);

    fn push(self, target: T) -> Self::Out {
        (target,)
    }
}

impl<T, T0> CombinLeft<T> for (T0,) {
    type Out = (T, T0);

    fn left(self, target: T) -> Self::Out {
        (target, self.0)
    }
}

impl<T, T0> CombinRight<T> for (T0,) {
    type Out = (T0, T);

    fn push(self, target: T) -> Self::Out {
        (self.0, target)
    }
}

include!(concat!(env!("OUT_DIR"), "/combin.rs"));

