//! Cloned & Copied

pub trait TupleCloned {
    type Output;

    fn cloned(self) -> Self::Output;
}

pub trait TupleCopied {
    type Output;

    fn copied(self) -> Self::Output;
}

impl<T: Clone> TupleCloned for &T {
    type Output = T;

    fn cloned(self) -> Self::Output {
        self.clone()
    }
}

impl<T: Copy + Clone> TupleCopied for &T {
    type Output = T;

    fn copied(self) -> Self::Output {
        *self
    }
}

impl TupleCloned for () {
    type Output = ();

    fn cloned(self) -> Self::Output {
        ()
    }
}

impl TupleCopied for () {
    type Output = ();

    fn copied(self) -> Self::Output {
        ()
    }
}

impl<'a, T: Clone> TupleCloned for (&'a T,) {
    type Output = (T,);

    fn cloned(self) -> Self::Output {
        (self.0.clone(),)
    }
}

impl<'a, T: Clone> TupleCloned for (&'a mut T,) {
    type Output = (T,);

    fn cloned(self) -> Self::Output {
        (self.0.clone(),)
    }
}

impl<'a, T: Copy> TupleCopied for (&'a T,) {
    type Output = (T,);

    fn copied(self) -> Self::Output {
        (*self.0,)
    }
}

impl<'a, T: Copy> TupleCopied for (&'a mut T,) {
    type Output = (T,);

    fn copied(self) -> Self::Output {
        (*self.0,)
    }
}

include!("./gen/clone.rs");

#[test]
fn test() {
    let a = (&1, &2, &3);
    let b = a.cloned();
    assert_eq!(b, (1, 2, 3))
}

#[test]
fn test2() {
    let a = 1;
    let b = a.cloned();
    assert_eq!(a, b)
}

#[test]
fn test3() {
    let a = &mut 1;
    let b = a.cloned();
    assert_eq!(*a, b)
}
