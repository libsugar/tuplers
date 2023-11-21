//! Sort tuples

use crate::{TupleGetMut, TupleSame};

/// Sort tuples using selection sort
pub trait TupleSortSelection<T> {
    /// Sort tuples using selection sort
    ///
    /// It has an `O(n2)` time complexity
    fn sort_selection(&mut self);
}

impl<T: PartialOrd, S: TupleSame<T> + TupleGetMut<Output = T>> TupleSortSelection<T> for S {
    #[inline]
    fn sort_selection(&mut self) {
        if sort_base(self, T::lt) {
            return;
        }
        selection_sort(self, T::lt)
    }
}

fn sort_base<T, S: TupleSame<T> + TupleGetMut<Output = T>>(v: &mut S, mut is_less: impl FnMut(&T, &T) -> bool) -> bool {
    if core::mem::size_of::<T>() == 0 {
        return true;
    }

    let len = v.arity();
    if len <= 1 {
        return true;
    }
    if len == 2 {
        let this = v as *mut S;
        let a = unsafe { (&mut *this).get_mut(0) };
        let b = unsafe { (&mut *this).get_mut(1) };

        if is_less(a, b) {
            return true;
        } else if is_less(b, a) {
            core::mem::swap(a, b);
            return true;
        } else {
            return true;
        }
    }

    false
}

fn selection_sort<T, S: TupleSame<T> + TupleGetMut<Output = T>>(v: &mut S, mut is_less: impl FnMut(&T, &T) -> bool) {
    let len = v.arity();
    for i in 0..(len - 1) {
        let mut min_index = i;

        for j in (i + 1)..len {
            if is_less(v.get(j), v.get(min_index)) {
                min_index = j;
            }
        }

        if min_index != i {
            let this = v as *mut S;
            let a = unsafe { (&mut *this).get_mut(i) };
            let b = unsafe { (&mut *this).get_mut(min_index) };
            core::mem::swap(a, b);
        }
    }
}

#[cfg(test)]
mod test_selection_sort {
    use super::*;

    #[test]
    fn test1() {
        let mut a = (6, 2, 6, 8, 0, 5);
        a.sort_selection();
        assert_eq!(a, (0, 2, 5, 6, 6, 8))
    }

    #[test]
    fn test2() {
        let mut a = (1, 2, 3, 4, 5, 6);
        a.sort_selection();
        assert_eq!(a, (1, 2, 3, 4, 5, 6))
    }

    #[test]
    fn test3() {
        let mut a = (2, 1);
        a.sort_selection();
        assert_eq!(a, (1, 2))
    }

    #[test]
    fn test4() {
        let mut a = (1, 2);
        a.sort_selection();
        assert_eq!(a, (1, 2))
    }

    #[test]
    fn test5() {
        let mut a = (1.5, 7.1, 3.0, 9.9, 0.3, 0.125, 0.1 + 0.2);
        a.sort_selection();
        assert_eq!(a, (0.125, 0.3, 0.1 + 0.2, 1.5, 3.0, 7.1, 9.9))
    }
}
