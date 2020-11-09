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

// impl<T0, T1> TupleFlatten for ((T0,), (T1,)) {
//     type OutTuple = (T0, T1);

//     fn flatten(self) -> Self::OutTuple {
//         ((self.0).0, (self.1).0)
//     }
// }

// impl<T0, T1, T2, T3> TupleFlatten for ((T0, T1), (T2, T3)) {
//     type OutTuple = (T0, T1, T2, T3);

//     fn flatten(self) -> Self::OutTuple {
//         ((self.0).0, (self.0).1, (self.1).0, (self.1).1)
//     }
// }

include!("./gen/flatten.rs");

#[test]
fn test() {
    let a = ((1, 2, 3), (4, 5, 6), (7, 8, 9));
    let b = a.flatten();
    assert_eq!(b, (1, 2, 3, 4, 5, 6, 7, 8, 9));
}
