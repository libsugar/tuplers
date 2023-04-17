/// Manually capture the variables required by the closure
pub fn capt0<T, R, F: Fn(&T) -> R>(v: T, f: F) -> impl Fn() -> R {
    move || f(&v)
}

/// Manually capture the variables required by the closure
pub fn capt1<T, A, R, F: Fn(&T, A) -> R>(v: T, f: F) -> impl Fn(A) -> R {
    move |a| f(&v, a)
}
