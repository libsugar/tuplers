// auto generated code, do not modify

impl<T> TupleSame<T> for (T, T) {}
impl<T> TupleSameV2<T> for (T, T) {}
impl<T0, T1> Tuple for (T0, T1) {
    fn arity(&self) -> usize {
        2
    }
}
impl<T0, T1> TupleV2 for (T0, T1) {
    const ARITY: usize = 2;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T) {}
impl<T0, T1, T2> Tuple for (T0, T1, T2) {
    fn arity(&self) -> usize {
        3
    }
}
impl<T0, T1, T2> TupleV2 for (T0, T1, T2) {
    const ARITY: usize = 3;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T) {}
impl<T0, T1, T2, T3> Tuple for (T0, T1, T2, T3) {
    fn arity(&self) -> usize {
        4
    }
}
impl<T0, T1, T2, T3> TupleV2 for (T0, T1, T2, T3) {
    const ARITY: usize = 4;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4> Tuple for (T0, T1, T2, T3, T4) {
    fn arity(&self) -> usize {
        5
    }
}
impl<T0, T1, T2, T3, T4> TupleV2 for (T0, T1, T2, T3, T4) {
    const ARITY: usize = 5;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5> Tuple for (T0, T1, T2, T3, T4, T5) {
    fn arity(&self) -> usize {
        6
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleV2 for (T0, T1, T2, T3, T4, T5) {
    const ARITY: usize = 6;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple for (T0, T1, T2, T3, T4, T5, T6) {
    fn arity(&self) -> usize {
        7
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleV2 for (T0, T1, T2, T3, T4, T5, T6) {
    const ARITY: usize = 7;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn arity(&self) -> usize {
        8
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    const ARITY: usize = 8;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn arity(&self) -> usize {
        9
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    const ARITY: usize = 9;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn arity(&self) -> usize {
        10
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    const ARITY: usize = 10;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn arity(&self) -> usize {
        11
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    const ARITY: usize = 11;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn arity(&self) -> usize {
        12
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    const ARITY: usize = 12;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn arity(&self) -> usize {
        13
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    const ARITY: usize = 13;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn arity(&self) -> usize {
        14
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    const ARITY: usize = 14;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn arity(&self) -> usize {
        15
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    const ARITY: usize = 15;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn arity(&self) -> usize {
        16
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    const ARITY: usize = 16;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    fn arity(&self) -> usize {
        17
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    const ARITY: usize = 17;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    fn arity(&self) -> usize {
        18
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    const ARITY: usize = 18;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    fn arity(&self) -> usize {
        19
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    const ARITY: usize = 19;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    fn arity(&self) -> usize {
        20
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    const ARITY: usize = 20;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    fn arity(&self) -> usize {
        21
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    const ARITY: usize = 21;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    fn arity(&self) -> usize {
        22
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    const ARITY: usize = 22;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    fn arity(&self) -> usize {
        23
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    const ARITY: usize = 23;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    fn arity(&self) -> usize {
        24
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    const ARITY: usize = 24;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    fn arity(&self) -> usize {
        25
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    const ARITY: usize = 25;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    fn arity(&self) -> usize {
        26
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    const ARITY: usize = 26;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    fn arity(&self) -> usize {
        27
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    const ARITY: usize = 27;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    fn arity(&self) -> usize {
        28
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    const ARITY: usize = 28;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    fn arity(&self) -> usize {
        29
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    const ARITY: usize = 29;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    fn arity(&self) -> usize {
        30
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    const ARITY: usize = 30;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    fn arity(&self) -> usize {
        31
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    const ARITY: usize = 31;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
impl<T> TupleSame<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T> TupleSameV2<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> Tuple for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    fn arity(&self) -> usize {
        32
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleV2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    const ARITY: usize = 32;
    type Item<const N: usize>
        = <Self as crate::TupleItemN<N>>::ItemN
    where
        Self: crate::TupleItemN<N>;
}
