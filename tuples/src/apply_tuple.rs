//! Apply tuple as args to fns

/// Tuple to fn meta
pub trait TupleFnMeta<R> {
    type DynFnOnce: ?Sized;
    type DynFnMut: ?Sized;
    type DynFn: ?Sized;
    type FnPtr;
}

/// Apply tuple meta
pub trait ApplyTupleMeta<Tuple> {
    type Output;
    type DynFnOnce: ?Sized;
    type DynFnMut: ?Sized;
    type DynFn: ?Sized;
    type FnPtr;
}

/// Apply tuple as args to FnOnce
pub trait ApplyTupleOnce<Tuple> {
    type Output;

    /// Apply tuple as args to FnOnce
    fn apply_tuple_once(self, args: Tuple) -> Self::Output;
}

/// Apply tuple as args to FnMut
pub trait ApplyTupleMut<Tuple>: ApplyTupleOnce<Tuple> {
    /// Apply tuple as args to FnMut
    fn apply_tuple_mut(&mut self, args: Tuple) -> Self::Output;
}

/// Apply tuple as args to Fn
pub trait ApplyTuple<Tuple>: ApplyTupleMut<Tuple> {
    /// Apply tuple as args to Fn
    fn apply_tuple(&self, args: Tuple) -> Self::Output;
}

include!("./gen/apply_tuple.rs");

#[test]
fn test1() {
    let a = (1, 2, 3);
    fn foo(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }
    let r = foo.apply_tuple(a);
    assert_eq!(r, 6)
}

#[test]
fn test2() {
    let a = (1, 2, 3);
    fn foo(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }
    let r = foo.apply_tuple_mut(a);
    assert_eq!(r, 6)
}

#[test]
fn test3() {
    let a = (1, 2, 3);
    fn foo(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }
    let r = foo.apply_tuple_once(a);
    assert_eq!(r, 6)
}
