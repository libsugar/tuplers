//! Miscellaneous

mod ref_or_mut {
    pub trait Internal {}

    impl<'a, T> Internal for &'a T {}
    impl<'a, T> Internal for &'a mut T {}
}

/// Allows passing immutable or mutable references
pub trait RefOrMut<'s, S = &'s Self>: ref_or_mut::Internal {
    type Target;

    /// Allows passing immutable or mutable references
    fn pass(&'s mut self) -> Self::Target;
}

impl<'s, 'a, T> RefOrMut<'s> for &'a T {
    type Target = &'s T;

    fn pass(&'s mut self) -> Self::Target {
        *self
    }
}

impl<'s, 'a, T> RefOrMut<'s> for &'a mut T {
    type Target = &'s mut T;

    fn pass(&'s mut self) -> Self::Target {
        *self
    }
}

/// Type erasure Result
pub trait AnyResult {
    type Ok;
    type Err;
}

impl<T, E> AnyResult for Result<T, E> {
    type Ok = T;
    type Err = E;
}
