//! Manually capture the variables required by the closure

/// Manually capture the variables required by the closure
pub fn capt_0<T, R, F: Fn(&T) -> R>(v: T, f: F) -> impl Fn() -> R {
    move || f(&v)
}

/// Manually capture the variables required by the closure
pub fn capt_mut_0<T, R, F: FnMut(&mut T) -> R>(mut v: T, mut f: F) -> impl FnMut() -> R {
    move || f(&mut v)
}

/// Manually capture the variables required by the closure
pub fn capt_once_0<T, R, F: FnOnce(T) -> R>(v: T, f: F) -> impl FnOnce() -> R {
    move || f(v)
}

include!("./gen/capt.rs");

#[test]
fn test1() {
    let a = capt_0(1, |a| *a);
    let r = a();
    assert_eq!(r, 1);
}

#[test]
fn test2() {
    let a = capt_1(2, |a, b| *a * b);
    let r = a(3);
    assert_eq!(r, 6);
}

#[test]
fn test3() {
    let mut a = capt_mut_1(2, |a, b| {
        *a *= b;
        *a
    });
    let r = a(3);
    assert_eq!(r, 6);
    let r = a(3);
    assert_eq!(r, 18);
}
