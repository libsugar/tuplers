//! Sort tuples

/// Sort tuples, currently use sort_selection
pub trait TupleSorted<T> {
    /// Sort tuples, currently use sort_selection
    fn sorted(self) -> Self;
}

/// Sort tuples in reverse order, currently use sort_selection
pub trait TupleSortedDesc<T> {
    /// Sort tuples in reverse order, currently use sort_selection
    fn sorted_desc(self) -> Self;
}

/// Sort tuples, currently use sort_selection
pub trait TupleSortedBy<T> {
    /// Sort tuples, currently use sort_selection
    fn sorted_by(self, cmp: impl FnMut(&T, &T) -> core::cmp::Ordering) -> Self;
}

/// Sort tuples, currently use sort_selection
pub trait TupleSortedByKey<T> {
    /// Sort tuples, currently use sort_selection
    fn sorted_by_key<K: PartialOrd>(self, selector: impl FnMut(&T) -> K) -> Self;
}

/// Sort tuples in reverse order, currently use sort_selection
pub trait TupleSortedByKeyDesc<T> {
    /// Sort tuples in reverse order, currently use sort_selection
    fn sorted_by_key_desc<K: PartialOrd>(self, selector: impl FnMut(&T) -> K) -> Self;
}

impl<T, S: TupleSort<T>> TupleSorted<T> for S {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}

impl<T, S: TupleSortDesc<T>> TupleSortedDesc<T> for S {
    fn sorted_desc(mut self) -> Self {
        self.sort_desc();
        self
    }
}

impl<T, S: TupleSortBy<T>> TupleSortedBy<T> for S {
    fn sorted_by(mut self, cmp: impl FnMut(&T, &T) -> core::cmp::Ordering) -> Self {
        self.sort_by(cmp);
        self
    }
}

impl<T, S: TupleSortByKey<T>> TupleSortedByKey<T> for S {
    fn sorted_by_key<K: PartialOrd>(mut self, selector: impl FnMut(&T) -> K) -> Self {
        self.sort_by_key(selector);
        self
    }
}

impl<T, S: TupleSortByKeyDesc<T>> TupleSortedByKeyDesc<T> for S {
    fn sorted_by_key_desc<K: PartialOrd>(mut self, selector: impl FnMut(&T) -> K) -> Self {
        self.sort_by_key_desc(selector);
        self
    }
}

/// Sort tuples, currently an alias for sort_selection
pub trait TupleSort<T> {
    /// Sort tuples, currently an alias for sort_selection
    fn sort(&mut self);
}

/// Sort tuples in reverse order, currently an alias for sort_selection
pub trait TupleSortDesc<T> {
    /// Sort tuples in reverse order, currently an alias for sort_selection
    fn sort_desc(&mut self);
}

/// Sort tuples, currently an alias for sort_selection
pub trait TupleSortBy<T> {
    /// Sort tuples, currently an alias for sort_selection
    fn sort_by(&mut self, cmp: impl FnMut(&T, &T) -> core::cmp::Ordering);
}

/// Sort tuples, currently an alias for sort_selection
pub trait TupleSortByKey<T> {
    /// Sort tuples, currently an alias for sort_selection
    fn sort_by_key<K: PartialOrd>(&mut self, selector: impl FnMut(&T) -> K);
}

/// Sort tuples in reverse order, currently an alias for sort_selection
pub trait TupleSortByKeyDesc<T> {
    /// Sort tuples in reverse order, currently an alias for sort_selection
    fn sort_by_key_desc<K: PartialOrd>(&mut self, selector: impl FnMut(&T) -> K);
}

impl<T, S: TupleSortSelection<T>> TupleSort<T> for S {
    fn sort(&mut self) {
        self.sort_selection();
    }
}

impl<T, S: TupleSortDescSelection<T>> TupleSortDesc<T> for S {
    fn sort_desc(&mut self) {
        self.sort_desc_selection();
    }
}

impl<T, S: TupleSortBySelection<T>> TupleSortBy<T> for S {
    fn sort_by(&mut self, cmp: impl FnMut(&T, &T) -> core::cmp::Ordering) {
        self.sort_by_selection(cmp);
    }
}

impl<T, S: TupleSortByKeySelection<T>> TupleSortByKey<T> for S {
    fn sort_by_key<K: PartialOrd>(&mut self, selector: impl FnMut(&T) -> K) {
        self.sort_by_key_selection(selector);
    }
}

impl<T, S: TupleSortByKeyDescSelection<T>> TupleSortByKeyDesc<T> for S {
    fn sort_by_key_desc<K: PartialOrd>(&mut self, selector: impl FnMut(&T) -> K) {
        self.sort_by_key_desc_selection(selector);
    }
}

pub use algorithms::selection::*;
/// Sorting algorithms
pub mod algorithms {
    /// Selection sort, it has an `O(n2)` time complexity
    pub mod selection {
        use crate::{TupleSame, TupleSwap};

        /// Sort tuples using selection sort
        pub trait TupleSortSelection<T> {
            /// Sort tuples using selection sort
            ///
            /// It has an `O(n2)` time complexity
            fn sort_selection(&mut self);
        }

        /// Sort tuples in reverse order using selection sort
        pub trait TupleSortDescSelection<T> {
            /// Sort tuples in reverse order using selection sort
            ///
            /// It has an `O(n2)` time complexity
            fn sort_desc_selection(&mut self);
        }

        /// Sort tuples using selection sort
        pub trait TupleSortBySelection<T> {
            /// Sort tuples using selection sort
            ///
            /// It has an `O(n2)` time complexity
            fn sort_by_selection(&mut self, cmp: impl FnMut(&T, &T) -> core::cmp::Ordering);
        }

        /// Sort tuples using selection sort
        pub trait TupleSortByKeySelection<T> {
            /// Sort tuples using selection sort
            ///
            /// It has an `O(n2)` time complexity
            fn sort_by_key_selection<K: PartialOrd>(&mut self, selector: impl FnMut(&T) -> K);
        }

        /// Sort tuples in reverse order  using selection sort
        pub trait TupleSortByKeyDescSelection<T> {
            /// Sort tuples in reverse order using selection sort
            ///
            /// It has an `O(n2)` time complexity
            fn sort_by_key_desc_selection<K: PartialOrd>(&mut self, selector: impl FnMut(&T) -> K);
        }

        impl<T: PartialOrd, S: TupleSame<T> + TupleSwap<Output = T>> TupleSortSelection<T> for S {
            #[inline]
            fn sort_selection(&mut self) {
                if sort_base(self, T::lt) {
                    return;
                }
                selection_sort(self, T::lt)
            }
        }

        impl<T: PartialOrd, S: TupleSame<T> + TupleSwap<Output = T>> TupleSortDescSelection<T> for S {
            #[inline]
            fn sort_desc_selection(&mut self) {
                if sort_base(self, T::gt) {
                    return;
                }
                selection_sort(self, T::gt)
            }
        }

        impl<T, S: TupleSame<T> + TupleSwap<Output = T>> TupleSortBySelection<T> for S {
            #[inline]
            fn sort_by_selection(&mut self, mut cmp: impl FnMut(&T, &T) -> core::cmp::Ordering) {
                if sort_base(self, |a, b| matches!(cmp(a, b), core::cmp::Ordering::Less)) {
                    return;
                }
                selection_sort(self, |a, b| matches!(cmp(a, b), core::cmp::Ordering::Less))
            }
        }

        impl<T, S: TupleSame<T> + TupleSwap<Output = T>> TupleSortByKeySelection<T> for S {
            #[inline]
            fn sort_by_key_selection<K: PartialOrd>(&mut self, mut selector: impl FnMut(&T) -> K) {
                if sort_base(self, |a, b| selector(a) < selector(b)) {
                    return;
                }
                selection_sort(self, |a, b| selector(a) < selector(b))
            }
        }

        impl<T, S: TupleSame<T> + TupleSwap<Output = T>> TupleSortByKeyDescSelection<T> for S {
            #[inline]
            fn sort_by_key_desc_selection<K: PartialOrd>(&mut self, mut selector: impl FnMut(&T) -> K) {
                if sort_base(self, |a, b| selector(a) > selector(b)) {
                    return;
                }
                selection_sort(self, |a, b| selector(a) > selector(b))
            }
        }

        fn sort_base<T, S: TupleSame<T> + TupleSwap<Output = T>>(v: &mut S, mut is_less: impl FnMut(&T, &T) -> bool) -> bool {
            if core::mem::size_of::<T>() == 0 {
                return true;
            }

            let len = v.arity();
            if len <= 1 {
                return true;
            }
            if len == 2 {
                let a = v.get(0);
                let b = v.get(1);

                if is_less(a, b) {
                    return true;
                } else if is_less(b, a) {
                    v.swap(0, 1);
                    return true;
                } else {
                    return true;
                }
            }

            false
        }

        fn selection_sort<T, S: TupleSame<T> + TupleSwap<Output = T>>(v: &mut S, mut is_less: impl FnMut(&T, &T) -> bool) {
            let len = v.arity();
            for i in 0..(len - 1) {
                let mut min_index = i;

                for j in (i + 1)..len {
                    if is_less(v.get(j), v.get(min_index)) {
                        min_index = j;
                    }
                }

                if min_index != i {
                    v.swap(i, min_index);
                }
            }
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

    #[test]
    fn test6() {
        let mut a = (6, 2, 6, 8, 0, 5);
        a.sort_by(|a, b| if a > b { core::cmp::Ordering::Less } else { core::cmp::Ordering::Greater });
        assert_eq!(a, (8, 6, 6, 5, 2, 0))
    }

    #[test]
    fn test7() {
        let mut a = ((6, 2), (6, 8), (0, 5));
        a.sort_by_key(|a| a.1);
        assert_eq!(a, ((6, 2), (0, 5), (6, 8)))
    }

    #[test]
    fn test8() {
        let mut a = (6, 2, 6, 8, 0, 5);
        a.sort_by(|a, b| a.cmp(b));
        assert_eq!(a, (0, 2, 5, 6, 6, 8))
    }

    #[test]
    fn test9() {
        let a = (6, 2, 6, 8, 0, 5);
        let a = a.sorted();
        assert_eq!(a, (0, 2, 5, 6, 6, 8))
    }

    #[test]
    fn test10() {
        let mut a = (6, 2, 6, 8, 0, 5);
        a.sort_desc();
        assert_eq!(a, (8, 6, 6, 5, 2, 0))
    }

    #[test]
    fn test11() {
        let mut a = ((6, 2), (6, 8), (0, 5));
        a.sort_by_key_desc(|a| a.1);
        assert_eq!(a, ((6, 8), (0, 5), (6, 2)))
    }
}
