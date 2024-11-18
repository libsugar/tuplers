// auto generated code, do not modify

#[doc = "Split to tuple by 2"]
pub trait TupleSplitToTupleBy2 {
    type OutTuple;
    #[doc = "Split to tuple by 2"]
    fn split_to_tuple_by_2(self) -> Self::OutTuple;
}
impl<T0, T1> TupleSplitToTupleBy2 for (T0, T1) {
    type OutTuple = ((T0, T1),);
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1),)
    }
}
impl<T0, T1, T2> TupleSplitToTupleBy2 for (T0, T1, T2) {
    type OutTuple = ((T0, T1), (T2,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2,))
    }
}
impl<T0, T1, T2, T3> TupleSplitToTupleBy2 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1), (T2, T3));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3))
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1), (T2, T3), (T4,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4,))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30,));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy2 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31));
    fn split_to_tuple_by_2(self) -> Self::OutTuple {
        ((self.0, self.1), (self.2, self.3), (self.4, self.5), (self.6, self.7), (self.8, self.9), (self.10, self.11), (self.12, self.13), (self.14, self.15), (self.16, self.17), (self.18, self.19), (self.20, self.21), (self.22, self.23), (self.24, self.25), (self.26, self.27), (self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 3"]
pub trait TupleSplitToTupleBy3 {
    type OutTuple;
    #[doc = "Split to tuple by 3"]
    fn split_to_tuple_by_3(self) -> Self::OutTuple;
}
impl<T0, T1, T2> TupleSplitToTupleBy3 for (T0, T1, T2) {
    type OutTuple = ((T0, T1, T2),);
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2),)
    }
}
impl<T0, T1, T2, T3> TupleSplitToTupleBy3 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1, T2), (T3,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3,))
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2), (T3, T4));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29), (T30,));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy3 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29), (T30, T31));
    fn split_to_tuple_by_3(self) -> Self::OutTuple {
        ((self.0, self.1, self.2), (self.3, self.4, self.5), (self.6, self.7, self.8), (self.9, self.10, self.11), (self.12, self.13, self.14), (self.15, self.16, self.17), (self.18, self.19, self.20), (self.21, self.22, self.23), (self.24, self.25, self.26), (self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 4"]
pub trait TupleSplitToTupleBy4 {
    type OutTuple;
    #[doc = "Split to tuple by 4"]
    fn split_to_tuple_by_4(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3> TupleSplitToTupleBy4 for (T0, T1, T2, T3) {
    type OutTuple = ((T0, T1, T2, T3),);
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3),)
    }
}
impl<T0, T1, T2, T3, T4> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2, T3), (T4,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4,))
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28,));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy4 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_to_tuple_by_4(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3), (self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 5"]
pub trait TupleSplitToTupleBy5 {
    type OutTuple;
    #[doc = "Split to tuple by 5"]
    fn split_to_tuple_by_5(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4) {
    type OutTuple = ((T0, T1, T2, T3, T4),);
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4),)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5,));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10,));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15,));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20,));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25,));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29), (T30,));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy5 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29), (T30, T31));
    fn split_to_tuple_by_5(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4), (self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 6"]
pub trait TupleSplitToTupleBy6 {
    type OutTuple;
    #[doc = "Split to tuple by 6"]
    fn split_to_tuple_by_6(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5),);
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6,));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12,));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18,));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24,));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29), (T30,));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy6 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_to_tuple_by_6(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5), (self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 7"]
pub trait TupleSplitToTupleBy7 {
    type OutTuple;
    #[doc = "Split to tuple by 7"]
    fn split_to_tuple_by_7(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6),);
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7,));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14,));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21,));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28,));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy7 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_to_tuple_by_7(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6), (self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 8"]
pub trait TupleSplitToTupleBy8 {
    type OutTuple;
    #[doc = "Split to tuple by 8"]
    fn split_to_tuple_by_8(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7),);
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8,));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16,));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24,));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy8 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_8(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7), (self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 9"]
pub trait TupleSplitToTupleBy9 {
    type OutTuple;
    #[doc = "Split to tuple by 9"]
    fn split_to_tuple_by_9(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8),);
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9,));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18,));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27,));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy9 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30, T31));
    fn split_to_tuple_by_9(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8), (self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 10"]
pub trait TupleSplitToTupleBy10 {
    type OutTuple;
    #[doc = "Split to tuple by 10"]
    fn split_to_tuple_by_10(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9),);
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10,));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20,));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30,));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy10 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_to_tuple_by_10(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9), (self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 11"]
pub trait TupleSplitToTupleBy11 {
    type OutTuple;
    #[doc = "Split to tuple by 11"]
    fn split_to_tuple_by_11(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10),);
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11,));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22,));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy11 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_11(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10), (self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 12"]
pub trait TupleSplitToTupleBy12 {
    type OutTuple;
    #[doc = "Split to tuple by 12"]
    fn split_to_tuple_by_12(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11),);
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12,));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24,));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy12 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_12(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11), (self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 13"]
pub trait TupleSplitToTupleBy13 {
    type OutTuple;
    #[doc = "Split to tuple by 13"]
    fn split_to_tuple_by_13(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12),);
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13,));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26,));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy13 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_13(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12), (self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 14"]
pub trait TupleSplitToTupleBy14 {
    type OutTuple;
    #[doc = "Split to tuple by 14"]
    fn split_to_tuple_by_14(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13),);
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14,));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28,));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy14 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_to_tuple_by_14(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13), (self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 15"]
pub trait TupleSplitToTupleBy15 {
    type OutTuple;
    #[doc = "Split to tuple by 15"]
    fn split_to_tuple_by_15(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14),);
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15,));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30,));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy15 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_to_tuple_by_15(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14), (self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 16"]
pub trait TupleSplitToTupleBy16 {
    type OutTuple;
    #[doc = "Split to tuple by 16"]
    fn split_to_tuple_by_16(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15),);
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16,));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy16 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_16(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15), (self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 17"]
pub trait TupleSplitToTupleBy17 {
    type OutTuple;
    #[doc = "Split to tuple by 17"]
    fn split_to_tuple_by_17(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16),);
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17,));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy17 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), (T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_17(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16), (self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 18"]
pub trait TupleSplitToTupleBy18 {
    type OutTuple;
    #[doc = "Split to tuple by 18"]
    fn split_to_tuple_by_18(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17),);
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18,));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy18 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_18(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17), (self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 19"]
pub trait TupleSplitToTupleBy19 {
    type OutTuple;
    #[doc = "Split to tuple by 19"]
    fn split_to_tuple_by_19(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18),);
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19,));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy19 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), (T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_19(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18), (self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 20"]
pub trait TupleSplitToTupleBy20 {
    type OutTuple;
    #[doc = "Split to tuple by 20"]
    fn split_to_tuple_by_20(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19),);
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20,));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy20 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_20(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19), (self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 21"]
pub trait TupleSplitToTupleBy21 {
    type OutTuple;
    #[doc = "Split to tuple by 21"]
    fn split_to_tuple_by_21(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20),);
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21,));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy21 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_21(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20), (self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 22"]
pub trait TupleSplitToTupleBy22 {
    type OutTuple;
    #[doc = "Split to tuple by 22"]
    fn split_to_tuple_by_22(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21),);
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22,));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy22 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), (T22, T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_22(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21), (self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 23"]
pub trait TupleSplitToTupleBy23 {
    type OutTuple;
    #[doc = "Split to tuple by 23"]
    fn split_to_tuple_by_23(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22),);
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23,));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy23 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), (T23, T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_23(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22), (self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 24"]
pub trait TupleSplitToTupleBy24 {
    type OutTuple;
    #[doc = "Split to tuple by 24"]
    fn split_to_tuple_by_24(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23),);
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24,));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy24 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_24(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23), (self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 25"]
pub trait TupleSplitToTupleBy25 {
    type OutTuple;
    #[doc = "Split to tuple by 25"]
    fn split_to_tuple_by_25(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24),);
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25,));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy25 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_25(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24), (self.25, self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 26"]
pub trait TupleSplitToTupleBy26 {
    type OutTuple;
    #[doc = "Split to tuple by 26"]
    fn split_to_tuple_by_26(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25),);
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26,));
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27));
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28));
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29));
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30));
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy26 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), (T26, T27, T28, T29, T30, T31));
    fn split_to_tuple_by_26(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25), (self.26, self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 27"]
pub trait TupleSplitToTupleBy27 {
    type OutTuple;
    #[doc = "Split to tuple by 27"]
    fn split_to_tuple_by_27(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleSplitToTupleBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26),);
    fn split_to_tuple_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27,));
    fn split_to_tuple_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28));
    fn split_to_tuple_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29));
    fn split_to_tuple_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30));
    fn split_to_tuple_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy27 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), (T27, T28, T29, T30, T31));
    fn split_to_tuple_by_27(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26), (self.27, self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 28"]
pub trait TupleSplitToTupleBy28 {
    type OutTuple;
    #[doc = "Split to tuple by 28"]
    fn split_to_tuple_by_28(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleSplitToTupleBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27),);
    fn split_to_tuple_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28,));
    fn split_to_tuple_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29));
    fn split_to_tuple_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30));
    fn split_to_tuple_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy28 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), (T28, T29, T30, T31));
    fn split_to_tuple_by_28(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27), (self.28, self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 29"]
pub trait TupleSplitToTupleBy29 {
    type OutTuple;
    #[doc = "Split to tuple by 29"]
    fn split_to_tuple_by_29(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleSplitToTupleBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28),);
    fn split_to_tuple_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29,));
    fn split_to_tuple_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29, T30));
    fn split_to_tuple_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29, self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy29 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), (T29, T30, T31));
    fn split_to_tuple_by_29(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28), (self.29, self.30, self.31))
    }
}
#[doc = "Split to tuple by 30"]
pub trait TupleSplitToTupleBy30 {
    type OutTuple;
    #[doc = "Split to tuple by 30"]
    fn split_to_tuple_by_30(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleSplitToTupleBy30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29),);
    fn split_to_tuple_by_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30,));
    fn split_to_tuple_by_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30,))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy30 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), (T30, T31));
    fn split_to_tuple_by_30(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29), (self.30, self.31))
    }
}
#[doc = "Split to tuple by 31"]
pub trait TupleSplitToTupleBy31 {
    type OutTuple;
    #[doc = "Split to tuple by 31"]
    fn split_to_tuple_by_31(self) -> Self::OutTuple;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleSplitToTupleBy31 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30),);
    fn split_to_tuple_by_31(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30),)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleSplitToTupleBy31 for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), (T31,));
    fn split_to_tuple_by_31(self) -> Self::OutTuple {
        ((self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30), (self.31,))
    }
}
