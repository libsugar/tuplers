//! Tuple Call

pub trait Tuple0Call {
    fn call<F: FnOnce() -> O, O>(self, f: F) -> O;
}

impl Tuple0Call for () {
    fn call<F: FnOnce() -> O, O>(self, f: F) -> O {
        f()
    }
}

pub trait Tuple1Call<T> {
    fn call<F: FnOnce(T) -> O, O>(self, f: F) -> O;
}

impl<T> Tuple1Call<T> for (T,) {
    fn call<F: FnOnce(T) -> O, O>(self, f: F) -> O {
        f(self.0)
    }
}

include!("./gen/tuple_call.rs");

#[test]
fn test() {
    let o = (1, 2, 3).call(|a, b, c| a + b + c);
    assert_eq!(o, 6);
}
