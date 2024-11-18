// auto generated code, do not modify

#[doc = "Split at 1"]
pub trait TupleSplitAt1 {
    type OutTuple;
    #[doc = "Split at 1"]
    fn split_at_1(self) -> Self::OutTuple;
}
impl<T0, T1> TupleSplitAt1 for (T0, T1) {
    type OutTuple = (T0, T1);
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, self.1)
    }
}
impl<T0, T1, T2> TupleSplitAt1 for (T0, T1, T2) {
    type OutTuple = (T0, (T1, T2));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2))
    }
}
impl<T0, T1, T2, T3> TupleSplitAt1 for (T0, T1, T2, T3) {
    type OutTuple = (T0, (T1, T2, T3));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3))
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitAt1 for (T0, T1, T2, T3, T4) {
    type OutTuple = (T0, (T1, T2, T3, T4));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt1 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (T0, (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_1(self) -> Self::OutTuple {
        (self.0, (self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 2"]
pub trait TupleSplitAt2 {
    type OutTuple;
    #[doc = "Split at 2"]
    fn split_at_2(self) -> Self::OutTuple;
}
impl<T0, T1, T2> TupleSplitAt2 for (T0, T1, T2) {
    type OutTuple = ((T0, T1), T2);
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), self.2)
    }
}
impl<T0, T1, T2, T3> TupleSplitAt2 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1), (T2, T3));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3))
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitAt2 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1), (T2, T3, T4));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1), (T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 3"]
pub trait TupleSplitAt3 {
    type OutTuple;
    #[doc = "Split at 3"]
    fn split_at_3(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3> TupleSplitAt3 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1, T2), T3);
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), self.3)
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitAt3 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2), (T3, T4));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 4"]
pub trait TupleSplitAt4 {
    type OutTuple;
    #[doc = "Split at 4"]
    fn split_at_4(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4> TupleSplitAt4 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2, T3), T4);
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), self.4)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 5"]
pub trait TupleSplitAt5 {
    type OutTuple;
    #[doc = "Split at 5"]
    fn split_at_5(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3, T4), T5);
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 6"]
pub trait TupleSplitAt6 {
    type OutTuple;
    #[doc = "Split at 6"]
    fn split_at_6(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), T6);
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 7"]
pub trait TupleSplitAt7 {
    type OutTuple;
    #[doc = "Split at 7"]
    fn split_at_7(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), T7);
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 8"]
pub trait TupleSplitAt8 {
    type OutTuple;
    #[doc = "Split at 8"]
    fn split_at_8(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), T8);
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 9"]
pub trait TupleSplitAt9 {
    type OutTuple;
    #[doc = "Split at 9"]
    fn split_at_9(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), T9);
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 10"]
pub trait TupleSplitAt10 {
    type OutTuple;
    #[doc = "Split at 10"]
    fn split_at_10(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), T10);
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 11"]
pub trait TupleSplitAt11 {
    type OutTuple;
    #[doc = "Split at 11"]
    fn split_at_11(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), T11);
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 12"]
pub trait TupleSplitAt12 {
    type OutTuple;
    #[doc = "Split at 12"]
    fn split_at_12(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), T12);
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 13"]
pub trait TupleSplitAt13 {
    type OutTuple;
    #[doc = "Split at 13"]
    fn split_at_13(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), T13);
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 14"]
pub trait TupleSplitAt14 {
    type OutTuple;
    #[doc = "Split at 14"]
    fn split_at_14(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), T14);
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 15"]
pub trait TupleSplitAt15 {
    type OutTuple;
    #[doc = "Split at 15"]
    fn split_at_15(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), T15);
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 16"]
pub trait TupleSplitAt16 {
    type OutTuple;
    #[doc = "Split at 16"]
    fn split_at_16(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), T16);
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 17"]
pub trait TupleSplitAt17 {
    type OutTuple;
    #[doc = "Split at 17"]
    fn split_at_17(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), T17);
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 18"]
pub trait TupleSplitAt18 {
    type OutTuple;
    #[doc = "Split at 18"]
    fn split_at_18(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), T18);
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 19"]
pub trait TupleSplitAt19 {
    type OutTuple;
    #[doc = "Split at 19"]
    fn split_at_19(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), T19);
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 20"]
pub trait TupleSplitAt20 {
    type OutTuple;
    #[doc = "Split at 20"]
    fn split_at_20(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), T20);
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 21"]
pub trait TupleSplitAt21 {
    type OutTuple;
    #[doc = "Split at 21"]
    fn split_at_21(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), T21);
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 22"]
pub trait TupleSplitAt22 {
    type OutTuple;
    #[doc = "Split at 22"]
    fn split_at_22(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), T22);
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 23"]
pub trait TupleSplitAt23 {
    type OutTuple;
    #[doc = "Split at 23"]
    fn split_at_23(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), T23);
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 24"]
pub trait TupleSplitAt24 {
    type OutTuple;
    #[doc = "Split at 24"]
    fn split_at_24(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), T24);
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_at_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 25"]
pub trait TupleSplitAt25 {
    type OutTuple;
    #[doc = "Split at 25"]
    fn split_at_25(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), T25);
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26));
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27));
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29));
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30));
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30, T31));
    fn split_at_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 26"]
pub trait TupleSplitAt26 {
    type OutTuple;
    #[doc = "Split at 26"]
    fn split_at_26(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitAt26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), T26);
    fn split_at_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27));
    fn split_at_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28));
    fn split_at_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29));
    fn split_at_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30));
    fn split_at_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30, T31));
    fn split_at_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 27"]
pub trait TupleSplitAt27 {
    type OutTuple;
    #[doc = "Split at 27"]
    fn split_at_27(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitAt27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), T27);
    fn split_at_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28));
    fn split_at_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29));
    fn split_at_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30));
    fn split_at_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30, T31));
    fn split_at_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 28"]
pub trait TupleSplitAt28 {
    type OutTuple;
    #[doc = "Split at 28"]
    fn split_at_28(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitAt28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), T28);
    fn split_at_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_at_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_at_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_at_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split at 29"]
pub trait TupleSplitAt29 {
    type OutTuple;
    #[doc = "Split at 29"]
    fn split_at_29(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitAt29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), T29);
    fn split_at_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29, T30));
    fn split_at_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29, T30, T31));
    fn split_at_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29, self.30, self.31))
    }
}
#[doc = "Split at 30"]
pub trait TupleSplitAt30 {
    type OutTuple;
    #[doc = "Split at 30"]
    fn split_at_30(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitAt30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), T30);
    fn split_at_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_at_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split at 31"]
pub trait TupleSplitAt31 {
    type OutTuple;
    #[doc = "Split at 31"]
    fn split_at_31(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitAt31 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), T31);
    fn split_at_31(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30), self.31)
    }
}
