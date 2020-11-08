//! Cloned & Copied

pub trait TupleCloned {
    type TupleOut;

    fn cloned(self) -> Self::TupleOut;
}

pub trait TupleCopied {
    type TupleOut;

    fn copied(self) -> Self::TupleOut;
}

impl TupleCloned for () {
    type TupleOut = ();

    fn cloned(self) -> Self::TupleOut {
        ()
    }
}

impl TupleCopied for () {
    type TupleOut = ();

    fn copied(self) -> Self::TupleOut {
        ()
    }
}

impl<'a, T: Clone> TupleCloned for (&'a T,) {
    type TupleOut = (T,);

    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(),)
    }
}

impl<'a, T: Clone> TupleCloned for (&'a mut T,) {
    type TupleOut = (T,);

    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(),)
    }
}

impl<'a, T: Copy> TupleCopied for (&'a T,) {
    type TupleOut = (T,);

    fn copied(self) -> Self::TupleOut {
        (*self.0,)
    }
}

impl<'a, T: Copy> TupleCopied for (&'a mut T,) {
    type TupleOut = (T,);

    fn copied(self) -> Self::TupleOut {
        (*self.0,)
    }
}

include!(concat!(env!("OUT_DIR"), "/cloned.rs"));

#[test]
fn test() {
    let a = (&1, &2, &3);
    let b = a.cloned();
    assert_eq!(b, (1, 2, 3))
}
