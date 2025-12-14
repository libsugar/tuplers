pub trait Param<'s, S = &'s Self> {
    type Target;

    fn pass(&'s mut self) -> Self::Target;
}

impl<'s, 'a, T> Param<'s> for &'a T {
    type Target = &'s T;

    fn pass(&'s mut self) -> Self::Target {
        *self
    }
}

impl<'s, 'a, T> Param<'s> for &'a mut T {
    type Target = &'s mut T;

    fn pass(&'s mut self) -> Self::Target {
        *self
    }
}
