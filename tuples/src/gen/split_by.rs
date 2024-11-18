// auto generated code, do not modify

#[doc = "Split by 2"]
pub trait TupleSplitBy2 {
    type OutTuple;
    #[doc = "Split by 2"]
    fn split_by_2(self) -> Self::OutTuple;
}
impl<T0, T1> TupleSplitBy2 for (T0, T1) {
    type OutTuple = (T0, T1);
    fn split_by_2(self) -> Self::OutTuple {
        (self.0, self.1)
    }
}
impl<T0, T1, T2> TupleSplitBy2 for (T0, T1, T2) {
    type OutTuple = ((T0, T1), T2);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), self.2)
    }
}
impl<T0, T1, T2, T3> TupleSplitBy2 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1), (T2, T3));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3))
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitBy2 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1), (T2, T3), T4);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), T6);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), T8);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), T10);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), T12);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), T14);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), T16);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), T18);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), T20);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), T22);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), T24);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), T26);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), T28);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), T30);
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31));
    fn split_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 3"]
pub trait TupleSplitBy3 {
    type OutTuple;
    #[doc = "Split by 3"]
    fn split_by_3(self) -> Self::OutTuple;
}
impl<T0, T1, T2> TupleSplitBy3 for (T0, T1, T2) {
    type OutTuple = (T0, T1, T2);
    fn split_by_3(self) -> Self::OutTuple {
        (self.0, self.1, self.2)
    }
}
impl<T0, T1, T2, T3> TupleSplitBy3 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1, T2), T3);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), self.3)
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitBy3 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2), (T3, T4));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), T6);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), T9);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), T12);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), T15);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), T18);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), T21);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), T24);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), T27);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29), T30);
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29), (T30, T31));
    fn split_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 4"]
pub trait TupleSplitBy4 {
    type OutTuple;
    #[doc = "Split by 4"]
    fn split_by_4(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3> TupleSplitBy4 for (T0, T1, T2, T3) {
    type OutTuple = (T0, T1, T2, T3);
    fn split_by_4(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3)
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitBy4 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2, T3), T4);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), T8);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), T12);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), T16);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), T20);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), T24);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), T28);
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 5"]
pub trait TupleSplitBy5 {
    type OutTuple;
    #[doc = "Split by 5"]
    fn split_by_5(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4> TupleSplitBy5 for (T0, T1, T2, T3, T4) {
    type OutTuple = (T0, T1, T2, T3, T4);
    fn split_by_5(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3, T4), T5);
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), T10);
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), T15);
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), T20);
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), T25);
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29), T30);
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29), (T30, T31));
    fn split_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 6"]
pub trait TupleSplitBy6 {
    type OutTuple;
    #[doc = "Split by 6"]
    fn split_by_6(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (T0, T1, T2, T3, T4, T5);
    fn split_by_6(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), T6);
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), T12);
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), T18);
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), T24);
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29), T30);
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 7"]
pub trait TupleSplitBy7 {
    type OutTuple;
    #[doc = "Split by 7"]
    fn split_by_7(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6);
    fn split_by_7(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), T7);
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), T14);
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), T21);
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), T28);
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 8"]
pub trait TupleSplitBy8 {
    type OutTuple;
    #[doc = "Split by 8"]
    fn split_by_8(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn split_by_8(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), T8);
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), T16);
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), T24);
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 9"]
pub trait TupleSplitBy9 {
    type OutTuple;
    #[doc = "Split by 9"]
    fn split_by_9(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn split_by_9(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), T9);
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), T18);
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), T27);
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30, T31));
    fn split_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 10"]
pub trait TupleSplitBy10 {
    type OutTuple;
    #[doc = "Split by 10"]
    fn split_by_10(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn split_by_10(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T10);
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), T20);
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), T30);
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 11"]
pub trait TupleSplitBy11 {
    type OutTuple;
    #[doc = "Split by 11"]
    fn split_by_11(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn split_by_11(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T11);
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), T22);
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 12"]
pub trait TupleSplitBy12 {
    type OutTuple;
    #[doc = "Split by 12"]
    fn split_by_12(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn split_by_12(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T12);
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), T24);
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 13"]
pub trait TupleSplitBy13 {
    type OutTuple;
    #[doc = "Split by 13"]
    fn split_by_13(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn split_by_13(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), T13);
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), T26);
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30, T31));
    fn split_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 14"]
pub trait TupleSplitBy14 {
    type OutTuple;
    #[doc = "Split by 14"]
    fn split_by_14(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn split_by_14(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), T14);
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), T28);
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 15"]
pub trait TupleSplitBy15 {
    type OutTuple;
    #[doc = "Split by 15"]
    fn split_by_15(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn split_by_15(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), T15);
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), T30);
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 16"]
pub trait TupleSplitBy16 {
    type OutTuple;
    #[doc = "Split by 16"]
    fn split_by_16(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn split_by_16(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), T16);
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 17"]
pub trait TupleSplitBy17 {
    type OutTuple;
    #[doc = "Split by 17"]
    fn split_by_17(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn split_by_17(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), T17);
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 18"]
pub trait TupleSplitBy18 {
    type OutTuple;
    #[doc = "Split by 18"]
    fn split_by_18(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn split_by_18(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), T18);
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 19"]
pub trait TupleSplitBy19 {
    type OutTuple;
    #[doc = "Split by 19"]
    fn split_by_19(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn split_by_19(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), T19);
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 20"]
pub trait TupleSplitBy20 {
    type OutTuple;
    #[doc = "Split by 20"]
    fn split_by_20(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn split_by_20(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), T20);
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 21"]
pub trait TupleSplitBy21 {
    type OutTuple;
    #[doc = "Split by 21"]
    fn split_by_21(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn split_by_21(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), T21);
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 22"]
pub trait TupleSplitBy22 {
    type OutTuple;
    #[doc = "Split by 22"]
    fn split_by_22(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn split_by_22(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), T22);
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 23"]
pub trait TupleSplitBy23 {
    type OutTuple;
    #[doc = "Split by 23"]
    fn split_by_23(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn split_by_23(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), T23);
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 24"]
pub trait TupleSplitBy24 {
    type OutTuple;
    #[doc = "Split by 24"]
    fn split_by_24(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn split_by_24(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), T24);
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 25"]
pub trait TupleSplitBy25 {
    type OutTuple;
    #[doc = "Split by 25"]
    fn split_by_25(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn split_by_25(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), T25);
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26));
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27));
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29));
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30));
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30, T31));
    fn split_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 26"]
pub trait TupleSplitBy26 {
    type OutTuple;
    #[doc = "Split by 26"]
    fn split_by_26(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn split_by_26(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), T26);
    fn split_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27));
    fn split_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28));
    fn split_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29));
    fn split_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30));
    fn split_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30, T31));
    fn split_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 27"]
pub trait TupleSplitBy27 {
    type OutTuple;
    #[doc = "Split by 27"]
    fn split_by_27(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn split_by_27(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), T27);
    fn split_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28));
    fn split_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29));
    fn split_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30));
    fn split_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30, T31));
    fn split_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 28"]
pub trait TupleSplitBy28 {
    type OutTuple;
    #[doc = "Split by 28"]
    fn split_by_28(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn split_by_28(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), T28);
    fn split_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split by 29"]
pub trait TupleSplitBy29 {
    type OutTuple;
    #[doc = "Split by 29"]
    fn split_by_29(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn split_by_29(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), T29);
    fn split_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29, T30));
    fn split_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29, T30, T31));
    fn split_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29, self.30, self.31))
    }
}
#[doc = "Split by 30"]
pub trait TupleSplitBy30 {
    type OutTuple;
    #[doc = "Split by 30"]
    fn split_by_30(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitBy30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn split_by_30(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), T30);
    fn split_by_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_by_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split by 31"]
pub trait TupleSplitBy31 {
    type OutTuple;
    #[doc = "Split by 31"]
    fn split_by_31(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitBy31 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn split_by_31(self) -> Self::OutTuple {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitBy31 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), T31);
    fn split_by_31(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30), self.31)
    }
}
