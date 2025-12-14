//! Get the nth item of the tuple

/// Get the nth item of the tuple
pub trait TupleGetV2N<const N: usize> {
    type Output;

    /// Get the nth item of the tuple
    fn get_n(&self) -> &Self::Output;
}

/// Get the nth item of the tuple
pub trait TupleGetMutV2N<const N: usize>: TupleGetV2N<N> {
    /// Get the nth item of the tuple
    fn get_n_mut(&mut self) -> &mut Self::Output;
}

/// Swaps two elements in the tuple
pub trait TupleSwapV2N<const A: usize, const B: usize>: TupleGetMutV2N<A> + TupleGetMutV2N<B> + TupleGetV2N<A, Output = <Self as TupleGetV2N<B>>::Output> {
    /// Swaps two elements in the tuple
    fn swap_n(&mut self);
}

impl<T, Item, const A: usize, const B: usize> TupleSwapV2N<A, B> for T
where
    T: TupleGetMutV2N<A> + TupleGetMutV2N<B> + TupleGetV2N<A, Output = Item> + TupleGetV2N<B, Output = Item>,
{
    fn swap_n(&mut self) {
        if A == B {
            return;
        }
        unsafe {
            let a = <Self as TupleGetMutV2N<A>>::get_n_mut(self) as *mut _;
            let b = <Self as TupleGetMutV2N<B>>::get_n_mut(self) as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
    }
}

/// Get the nth item of the tuple
pub trait TupleGetV2 {
    type Output<const N: usize>
    where
        Self: TupleGetV2N<N>;

    /// Get the nth item of the tuple
    fn get<const N: usize>(&self) -> &<Self as TupleGetV2>::Output<N>
    where
        Self: TupleGetV2N<N>;
}

/// Get the nth item of the tuple
pub trait TupleGetMutV2: TupleGetV2 {
    /// Get the nth item of the tuple
    fn get_mut<const N: usize>(&mut self) -> &mut <Self as TupleGetV2>::Output<N>
    where
        Self: TupleGetMutV2N<N>;
}

/// Swaps two elements in the tuple
pub trait TupleSwapV2: TupleGetMutV2 {
    /// Swaps two elements in the tuple
    fn swap<const A: usize, const B: usize>(&mut self)
    where
        Self: TupleSwapV2N<A, B>;
}

impl<T> TupleGetV2 for T {
    type Output<const N: usize>
        = <Self as TupleGetV2N<N>>::Output
    where
        Self: TupleGetV2N<N>;

    fn get<const N: usize>(&self) -> &Self::Output<N>
    where
        Self: TupleGetV2N<N>,
    {
        self.get_n()
    }
}

impl<T> TupleGetMutV2 for T {
    fn get_mut<const N: usize>(&mut self) -> &mut Self::Output<N>
    where
        Self: TupleGetMutV2N<N>,
    {
        self.get_n_mut()
    }
}

impl<T> TupleSwapV2 for T {
    fn swap<const A: usize, const B: usize>(&mut self)
    where
        Self: TupleSwapV2N<A, B>,
    {
        self.swap_n();
    }
}

include!("./gen/tuple_get_v2.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        let a = (1, 2, 3);
        let r: &i32 = a.get::<1>();
        assert_eq!(*r, 2);
    }

    #[test]
    fn test1() {
        let a = (1, '2', "3");
        let r: &char = a.get::<1>();
        assert_eq!(*r, '2');
    }

    #[test]
    fn test2() {
        let mut a = (1, '2', "3");
        *a.get_mut::<1>() = 'c';
        assert_eq!(a.1, 'c');
    }

    #[test]
    fn test3() {
        let mut a = (1, '2', "3", 6);
        a.swap::<0, 3>();
        assert_eq!(a.0, 6);
    }
}
