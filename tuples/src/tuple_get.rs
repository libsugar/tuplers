//! Get the nth item of the tuple

/// Get the nth item of the tuple
pub trait TupleGet {
    type Output;

    /// Get the nth item of the tuple
    fn get(&self, index: usize) -> &Self::Output;
    /// Get the nth item of the tuple
    fn try_get(&self, index: usize) -> Option<&Self::Output>;
}

/// Get the nth item of the tuple
pub trait TupleGetMut: TupleGet {
    /// Get the nth item of the tuple
    fn get_mut(&mut self, index: usize) -> &mut Self::Output;
    /// Get the nth item of the tuple
    fn try_get_mut(&mut self, index: usize) -> Option<&mut Self::Output>;
}

/// Swaps two elements in the tuple
pub trait TupleSwap: TupleGetMut {
    /// Swaps two elements in the tuple
    fn swap(&mut self, a: usize, b: usize);

    /// Swaps two elements in the tuple
    fn try_swap(&mut self, a: usize, b: usize) -> bool;
}

include!("./gen/tuple_get.rs");

#[test]
fn test() {
    let a = (1, 2, 3, 4, 5);
    assert_eq!(*a.get(2), 3);

    let mut a = (1, 2, 3, 4, 5);
    *a.get_mut(3) = 6;
    assert_eq!(a, (1, 2, 3, 6, 5));
}

#[test]
fn test_swap() {
    let mut a = (1, 2, 3, 4, 5);
    a.swap(1, 3);
    assert_eq!(a, (1, 4, 3, 2, 5));
}
