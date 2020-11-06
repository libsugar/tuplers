//! Mapping for tuples

include!(concat!(env!("OUT_DIR"), "/tuple_map.rs"));

/// Mapping for Tuple1
pub trait Tuple1Map<T> {
    /// Mapping for Tuple1
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U,);
}
impl<T> Tuple1Map<T> for (T,) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U,) {
        (f(self.0),)
    }
}
