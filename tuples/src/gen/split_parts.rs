// auto generated code, do not modify

#[doc = "Split into 2 parts"]
pub trait TupleSplit2Parts {
    type OutTuple;
    #[doc = "Split into 2 parts"]
    fn split_2_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3> TupleSplit2Parts for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1), (T2, T3));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3))
    }
}
impl<T0, T1, T2, T3, T4> TupleSplit2Parts for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2), (T3, T4));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit2Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_2_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 3 parts"]
pub trait TupleSplit3Parts {
    type OutTuple;
    #[doc = "Split into 3 parts"]
    fn split_3_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8), (T9, T10, T11, T12));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit3Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_3_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 4 parts"]
pub trait TupleSplit4Parts {
    type OutTuple;
    #[doc = "Split into 4 parts"]
    fn split_4_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8), (T9, T10, T11, T12), (T13, T14, T15, T16));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13), (T14, T15, T16, T17));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit4Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_4_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 5 parts"]
pub trait TupleSplit5Parts {
    type OutTuple;
    #[doc = "Split into 5 parts"]
    fn split_5_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12), (T13, T14, T15));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12), (self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13), (T14, T15, T16));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13), (self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14), (T15, T16, T17));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8), (T9, T10, T11, T12), (T13, T14, T15, T16), (T17, T18, T19, T20));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13), (T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18), (T19, T20, T21, T22));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit5Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30, T31));
    fn split_5_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 6 parts"]
pub trait TupleSplit6Parts {
    type OutTuple;
    #[doc = "Split into 6 parts"]
    fn split_6_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12), (T13, T14, T15), (T16, T17, T18));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12), (self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13), (T14, T15, T16), (T17, T18, T19));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13), (self.14, self.15, self.16), (self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18), (T19, T20, T21));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8), (T9, T10, T11, T12), (T13, T14, T15, T16), (T17, T18, T19, T20), (T21, T22, T23, T24));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13), (T14, T15, T16, T17), (T18, T19, T20, T21), (T22, T23, T24, T25));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18), (T19, T20, T21, T22), (T23, T24, T25, T26));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit6Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26), (T27, T28, T29, T30, T31));
    fn split_6_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 7 parts"]
pub trait TupleSplit7Parts {
    type OutTuple;
    #[doc = "Split into 7 parts"]
    fn split_7_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12), (T13, T14, T15), (T16, T17, T18), (T19, T20, T21));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12), (self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13), (T14, T15, T16), (T17, T18, T19), (T20, T21, T22));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13), (self.14, self.15, self.16), (self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18), (T19, T20, T21), (T22, T23, T24));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22), (T23, T24, T25));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22), (self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8), (T9, T10, T11, T12), (T13, T14, T15, T16), (T17, T18, T19, T20), (T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13), (T14, T15, T16, T17), (T18, T19, T20, T21), (T22, T23, T24, T25), (T26, T27, T28, T29));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18), (T19, T20, T21, T22), (T23, T24, T25, T26), (T27, T28, T29, T30));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit7Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_7_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 8 parts"]
pub trait TupleSplit8Parts {
    type OutTuple;
    #[doc = "Split into 8 parts"]
    fn split_8_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18), (T19, T20));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19), (T20, T21));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12), (T13, T14, T15), (T16, T17, T18), (T19, T20, T21), (T22, T23, T24));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12), (self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13), (T14, T15, T16), (T17, T18, T19), (T20, T21, T22), (T23, T24, T25));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13), (self.14, self.15, self.16), (self.17, self.18, self.19), (self.20, self.21, self.22), (self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18), (T19, T20, T21), (T22, T23, T24), (T25, T26, T27));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21), (self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22), (T23, T24, T25), (T26, T27, T28));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22), (self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26), (T27, T28, T29));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit8Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_8_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split into 9 parts"]
pub trait TupleSplit9Parts {
    type OutTuple;
    #[doc = "Split into 9 parts"]
    fn split_9_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19), (T20, T21), (T22, T23));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22), (T23, T24));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12), (T13, T14, T15), (T16, T17, T18), (T19, T20, T21), (T22, T23, T24), (T25, T26, T27));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12), (self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21), (self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13), (T14, T15, T16), (T17, T18, T19), (T20, T21, T22), (T23, T24, T25), (T26, T27, T28));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13), (self.14, self.15, self.16), (self.17, self.18, self.19), (self.20, self.21, self.22), (self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18), (T19, T20, T21), (T22, T23, T24), (T25, T26, T27), (T28, T29, T30));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21), (self.22, self.23, self.24), (self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit9Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22), (T23, T24, T25), (T26, T27, T28), (T29, T30, T31));
    fn split_9_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22), (self.23, self.24, self.25), (self.26, self.27, self.28), (self.29, self.30, self.31))
    }
}
#[doc = "Split into 10 parts"]
pub trait TupleSplit10Parts {
    type OutTuple;
    #[doc = "Split into 10 parts"]
    fn split_10_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22), (T23, T24), (T25, T26));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25), (T26, T27));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6), (T7, T8, T9), (T10, T11, T12), (T13, T14, T15), (T16, T17, T18), (T19, T20, T21), (T22, T23, T24), (T25, T26, T27), (T28, T29, T30));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6), (self.7, self.8, self.9), (self.10, self.11, self.12), (self.13, self.14, self.15), (self.16, self.17, self.18), (self.19, self.20, self.21), (self.22, self.23, self.24), (self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit10Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10), (T11, T12, T13), (T14, T15, T16), (T17, T18, T19), (T20, T21, T22), (T23, T24, T25), (T26, T27, T28), (T29, T30, T31));
    fn split_10_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10), (self.11, self.12, self.13), (self.14, self.15, self.16), (self.17, self.18, self.19), (self.20, self.21, self.22), (self.23, self.24, self.25), (self.26, self.27, self.28), (self.29, self.30, self.31))
    }
}
#[doc = "Split into 11 parts"]
pub trait TupleSplit11Parts {
    type OutTuple;
    #[doc = "Split into 11 parts"]
    fn split_11_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28), (T29, T30));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit11Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29), (T30, T31));
    fn split_11_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split into 12 parts"]
pub trait TupleSplit12Parts {
    type OutTuple;
    #[doc = "Split into 12 parts"]
    fn split_12_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28), (T29, T30));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit12Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31));
    fn split_12_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split into 13 parts"]
pub trait TupleSplit13Parts {
    type OutTuple;
    #[doc = "Split into 13 parts"]
    fn split_13_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28), (T29, T30));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit13Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31));
    fn split_13_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split into 14 parts"]
pub trait TupleSplit14Parts {
    type OutTuple;
    #[doc = "Split into 14 parts"]
    fn split_14_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplit14Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27));
    fn split_14_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplit14Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28));
    fn split_14_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit14Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_14_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit14Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28), (T29, T30));
    fn split_14_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit14Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31));
    fn split_14_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split into 15 parts"]
pub trait TupleSplit15Parts {
    type OutTuple;
    #[doc = "Split into 15 parts"]
    fn split_15_parts(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplit15Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_15_parts(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplit15Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4), (T5, T6), (T7, T8), (T9, T10), (T11, T12), (T13, T14), (T15, T16), (T17, T18), (T19, T20), (T21, T22), (T23, T24), (T25, T26), (T27, T28), (T29, T30));
    fn split_15_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4), (self.5, self.6), (self.7, self.8), (self.9, self.10), (self.11, self.12), (self.13, self.14), (self.15, self.16), (self.17, self.18), (self.19, self.20), (self.21, self.22), (self.23, self.24), (self.25, self.26), (self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplit15Parts for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31));
    fn split_15_parts(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30, self.31))
    }
}
