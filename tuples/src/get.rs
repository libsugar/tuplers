//! Get the nth item of the tuple

use crate::{HomogeneousTuple, Tuple, TupleItem, TupleNthItem};

/// Get the nth item of the tuple
pub trait TupleDynamicGet: HomogeneousTuple<Self::Output> {
    type Output;

    /// Get the nth item of the tuple
    fn dyn_get(&self, index: usize) -> Option<&Self::Output>;
}

/// Get the nth item of the tuple
pub trait TupleDynamicGetMut: TupleDynamicGet {
    /// Get the nth item of the tuple
    fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output>;
}

/// Swaps two elements in the tuple
pub trait TupleDynamicSwap: TupleDynamicGetMut {
    /// Swaps two elements in the tuple
    fn dyn_swap(&mut self, a: usize, b: usize) -> bool;
}

/// Get the nth item of the tuple
pub trait TupleGetN<const N: usize>: Tuple<Item<N> = Self::Output> + TupleItem<N, ItemN = Self::Output> {
    type Output;

    /// Get the nth item of the tuple
    fn get_n(&self) -> &Self::Output;
}

/// Get the nth item of the tuple
pub trait TupleGetMutN<const N: usize>: TupleGetN<N> {
    /// Get the nth item of the tuple
    fn get_n_mut(&mut self) -> &mut Self::Output;
}

/// Swaps two elements in the tuple
pub trait TupleSwapN<const A: usize, const B: usize>: TupleGetMutN<A> + TupleGetMutN<B> + TupleGetN<A, Output = <Self as TupleGetN<B>>::Output> {
    /// Swaps two elements in the tuple
    fn swap_n(&mut self);
}

impl<T, Item, const A: usize, const B: usize> TupleSwapN<A, B> for T
where
    T: TupleGetMutN<A> + TupleGetMutN<B> + TupleGetN<A, Output = Item> + TupleGetN<B, Output = Item>,
{
    fn swap_n(&mut self) {
        if A == B {
            return;
        }
        unsafe {
            let a = <Self as TupleGetMutN<A>>::get_n_mut(self) as *mut _;
            let b = <Self as TupleGetMutN<B>>::get_n_mut(self) as *mut _;
            core::mem::swap(&mut *a, &mut *b);
        }
    }
}

/// Get the nth item of the tuple
pub trait TupleGet {
    type Output<const N: usize>
    where
        Self: TupleGetN<N>;

    /// Get the nth item of the tuple
    fn get<const N: usize>(&self) -> &<Self as TupleGet>::Output<N>
    where
        Self: TupleGetN<N>;
}

/// Get the nth item of the tuple
pub trait TupleGetMut: TupleGet {
    /// Get the nth item of the tuple
    fn get_mut<const N: usize>(&mut self) -> &mut <Self as TupleGet>::Output<N>
    where
        Self: TupleGetMutN<N>;
}

/// Swaps two elements in the tuple
pub trait TupleSwap: TupleGetMut {
    /// Swaps two elements in the tuple
    fn swap<const A: usize, const B: usize>(&mut self)
    where
        Self: TupleSwapN<A, B>;
}

include!("./gen/get_dyn.rs");
include!("./gen/get_const.rs");

#[cfg(test)]
mod test_dyn {
    use crate::*;

    #[test]
    fn test() {
        let a = (1, 2, 3, 4, 5);
        assert_eq!(*a.dyn_get(2).unwrap(), 3);

        let mut a = (1, 2, 3, 4, 5);
        *a.dyn_get_mut(3).unwrap() = 6;
        assert_eq!(a, (1, 2, 3, 6, 5));
    }

    #[test]
    fn test_swap() {
        let mut a = (1, 2, 3, 4, 5);
        a.dyn_swap(1, 3);
        assert_eq!(a, (1, 4, 3, 2, 5));
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    trait Some {
        fn get(&self) -> i32;
    }

    struct Foo;

    impl Some for Foo {
        fn get(&self) -> i32 {
            1
        }
    }

    #[test]
    fn foo() {
        let x = Foo;
        let y = x.get();
        assert_eq!(y, 1);
    }

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
