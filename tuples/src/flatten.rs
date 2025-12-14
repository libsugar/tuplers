//! Flatten

/// Flatten tuple once
pub trait TupleFlatten {
    type OutTuple;

    /// Flatten tuple once
    fn flatten(self) -> Self::OutTuple;
}

impl TupleFlatten for () {
    type OutTuple = ();

    fn flatten(self) -> Self::OutTuple {
        self
    }
}

impl<T> TupleFlatten for (T,) {
    type OutTuple = T;

    fn flatten(self) -> Self::OutTuple {
        self.0
    }
}

include!("./gen/flatten.rs");

#[test]
fn test() {
    let a = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    let b = a.flatten();
    assert_eq!(b, (1, 2, 3, 4, 5, 6, 7, 8, 9));
}
