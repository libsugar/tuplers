// auto generated code, do not modify

impl<T0, T1> TupleTranspose for Option<(T0, T1)> {
    type OutTuple = (Option<T0>, Option<T1>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1)),
            None => (None, None),
        }
    }
}
impl<T0, T1, T2> TupleTranspose for Option<(T0, T1, T2)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2)),
            None => (None, None, None),
        }
    }
}
impl<T0, T1, T2, T3> TupleTranspose for Option<(T0, T1, T2, T3)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3)),
            None => (None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4> TupleTranspose for Option<(T0, T1, T2, T3, T4)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4)),
            None => (None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5)),
            None => (None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6)),
            None => (None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7)),
            None => (None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8)),
            None => (None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9)),
            None => (None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10)),
            None => (None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25), Some(v.26)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25), Some(v.26), Some(v.27)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25), Some(v.26), Some(v.27), Some(v.28)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25), Some(v.26), Some(v.27), Some(v.28), Some(v.29)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25), Some(v.26), Some(v.27), Some(v.28), Some(v.29), Some(v.30)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleTranspose for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)> {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>, Option<T31>);
    fn transpose(self) -> Self::OutTuple {
        match self {
            Some(v) => (Some(v.0), Some(v.1), Some(v.2), Some(v.3), Some(v.4), Some(v.5), Some(v.6), Some(v.7), Some(v.8), Some(v.9), Some(v.10), Some(v.11), Some(v.12), Some(v.13), Some(v.14), Some(v.15), Some(v.16), Some(v.17), Some(v.18), Some(v.19), Some(v.20), Some(v.21), Some(v.22), Some(v.23), Some(v.24), Some(v.25), Some(v.26), Some(v.27), Some(v.28), Some(v.29), Some(v.30), Some(v.31)),
            None => (None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None),
        }
    }
}
impl<T0, T1> TupleTranspose for (Option<T0>, Option<T1>) {
    type OutTuple = Option<(T0, T1)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1)) => Some((v0, v1)),
            _ => None,
        }
    }
}
impl<T0, T1, T2> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>) {
    type OutTuple = Option<(T0, T1, T2)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2)) => Some((v0, v1, v2)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>) {
    type OutTuple = Option<(T0, T1, T2, T3)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3)) => Some((v0, v1, v2, v3)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4)) => Some((v0, v1, v2, v3, v4)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5)) => Some((v0, v1, v2, v3, v4, v5)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6)) => Some((v0, v1, v2, v3, v4, v5, v6)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7)) => Some((v0, v1, v2, v3, v4, v5, v6, v7)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25), Some(v26)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25), Some(v26), Some(v27)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25), Some(v26), Some(v27), Some(v28)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25), Some(v26), Some(v27), Some(v28), Some(v29)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25), Some(v26), Some(v27), Some(v28), Some(v29), Some(v30)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)),
            _ => None,
        }
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleTranspose for (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>, Option<T31>) {
    type OutTuple = Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)>;
    fn transpose(self) -> Self::OutTuple {
        match self {
            (Some(v0), Some(v1), Some(v2), Some(v3), Some(v4), Some(v5), Some(v6), Some(v7), Some(v8), Some(v9), Some(v10), Some(v11), Some(v12), Some(v13), Some(v14), Some(v15), Some(v16), Some(v17), Some(v18), Some(v19), Some(v20), Some(v21), Some(v22), Some(v23), Some(v24), Some(v25), Some(v26), Some(v27), Some(v28), Some(v29), Some(v30), Some(v31)) => Some((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)),
            _ => None,
        }
    }
}
impl<E, T0, T1> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>) {
    type OutTuple = Result<(T0, T1), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1) = self;
        Ok((v0?, v1?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult2<E0, E1> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1> TupleTransposeResult2<E0, E1> for (Result<T0, E0>, Result<T1, E1>) {
    type OutTuple<Eo> = Result<(T0, T1), Eo>;
    fn transpose<Eo: From<E0> + From<E1>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1) = self;
        Ok((v0?, v1?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr2<E0, E1> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1> TupleTransposeResultMapErr2<E0, E1> for (Result<T0, E0>, Result<T1, E1>) {
    type OutTuple<Eo> = Result<(T0, T1), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>) {
    type OutTuple = Result<(T0, T1, T2), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2) = self;
        Ok((v0?, v1?, v2?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult3<E0, E1, E2> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2> TupleTransposeResult3<E0, E1, E2> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>) {
    type OutTuple<Eo> = Result<(T0, T1, T2), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2) = self;
        Ok((v0?, v1?, v2?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr3<E0, E1, E2> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2> TupleTransposeResultMapErr3<E0, E1, E2> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>) {
    type OutTuple<Eo> = Result<(T0, T1, T2), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>) {
    type OutTuple = Result<(T0, T1, T2, T3), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3) = self;
        Ok((v0?, v1?, v2?, v3?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult4<E0, E1, E2, E3> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3> TupleTransposeResult4<E0, E1, E2, E3> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3) = self;
        Ok((v0?, v1?, v2?, v3?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr4<E0, E1, E2, E3> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3> TupleTransposeResultMapErr4<E0, E1, E2, E3> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4) = self;
        Ok((v0?, v1?, v2?, v3?, v4?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult5<E0, E1, E2, E3, E4> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4> TupleTransposeResult5<E0, E1, E2, E3, E4> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4) = self;
        Ok((v0?, v1?, v2?, v3?, v4?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr5<E0, E1, E2, E3, E4> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4> TupleTransposeResultMapErr5<E0, E1, E2, E3, E4> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult6<E0, E1, E2, E3, E4, E5> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5> TupleTransposeResult6<E0, E1, E2, E3, E4, E5> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr6<E0, E1, E2, E3, E4, E5> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5> TupleTransposeResultMapErr6<E0, E1, E2, E3, E4, E5> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult7<E0, E1, E2, E3, E4, E5, E6> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6> TupleTransposeResult7<E0, E1, E2, E3, E4, E5, E6> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr7<E0, E1, E2, E3, E4, E5, E6> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6> TupleTransposeResultMapErr7<E0, E1, E2, E3, E4, E5, E6> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult8<E0, E1, E2, E3, E4, E5, E6, E7> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7> TupleTransposeResult8<E0, E1, E2, E3, E4, E5, E6, E7> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr8<E0, E1, E2, E3, E4, E5, E6, E7> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7> TupleTransposeResultMapErr8<E0, E1, E2, E3, E4, E5, E6, E7> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult9<E0, E1, E2, E3, E4, E5, E6, E7, E8> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8> TupleTransposeResult9<E0, E1, E2, E3, E4, E5, E6, E7, E8> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr9<E0, E1, E2, E3, E4, E5, E6, E7, E8> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8> TupleTransposeResultMapErr9<E0, E1, E2, E3, E4, E5, E6, E7, E8> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult10<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9> TupleTransposeResult10<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr10<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9> TupleTransposeResultMapErr10<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult11<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10> TupleTransposeResult11<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr11<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10> TupleTransposeResultMapErr11<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult12<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11> TupleTransposeResult12<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr12<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11> TupleTransposeResultMapErr12<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult13<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12> TupleTransposeResult13<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr13<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12> TupleTransposeResultMapErr13<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult14<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13> TupleTransposeResult14<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr14<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13> TupleTransposeResultMapErr14<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult15<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14> TupleTransposeResult15<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr15<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14> TupleTransposeResultMapErr15<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult16<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15> TupleTransposeResult16<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr16<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15> TupleTransposeResultMapErr16<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult17<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16> TupleTransposeResult17<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr17<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16> TupleTransposeResultMapErr17<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult18<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17> TupleTransposeResult18<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr18<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17> TupleTransposeResultMapErr18<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult19<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18> TupleTransposeResult19<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr19<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18> TupleTransposeResultMapErr19<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult20<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19> TupleTransposeResult20<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr20<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19> TupleTransposeResultMapErr20<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult21<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20> TupleTransposeResult21<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr21<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20> TupleTransposeResultMapErr21<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult22<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21> TupleTransposeResult22<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr22<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21> TupleTransposeResultMapErr22<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult23<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22> TupleTransposeResult23<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr23<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22> TupleTransposeResultMapErr23<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult24<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23> TupleTransposeResult24<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr24<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23> TupleTransposeResultMapErr24<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult25<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24> TupleTransposeResult25<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr25<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24> TupleTransposeResultMapErr25<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult26<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25> TupleTransposeResult26<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr26<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25> TupleTransposeResultMapErr26<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult27<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26> TupleTransposeResult27<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr27<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26> TupleTransposeResultMapErr27<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
            match v26 {
                Ok(v) => v,
                Err(e) => Err(f26(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult28<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27> TupleTransposeResult28<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr28<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27> TupleTransposeResultMapErr28<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
            match v26 {
                Ok(v) => v,
                Err(e) => Err(f26(e))?,
            },
            match v27 {
                Ok(v) => v,
                Err(e) => Err(f27(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult29<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28> TupleTransposeResult29<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr29<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28> TupleTransposeResultMapErr29<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
            match v26 {
                Ok(v) => v,
                Err(e) => Err(f26(e))?,
            },
            match v27 {
                Ok(v) => v,
                Err(e) => Err(f27(e))?,
            },
            match v28 {
                Ok(v) => v,
                Err(e) => Err(f28(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult30<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29> TupleTransposeResult30<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr30<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo, f29: impl FnOnce(E29) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29> TupleTransposeResultMapErr30<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo, f29: impl FnOnce(E29) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
            match v26 {
                Ok(v) => v,
                Err(e) => Err(f26(e))?,
            },
            match v27 {
                Ok(v) => v,
                Err(e) => Err(f27(e))?,
            },
            match v28 {
                Ok(v) => v,
                Err(e) => Err(f28(e))?,
            },
            match v29 {
                Ok(v) => v,
                Err(e) => Err(f29(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>, Result<T30, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?, v30?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult31<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29> + From<E30>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29, E30, T30> TupleTransposeResult31<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>, Result<T30, E30>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29> + From<E30>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?, v30?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr31<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo, f29: impl FnOnce(E29) -> Eo, f30: impl FnOnce(E30) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29, E30, T30> TupleTransposeResultMapErr31<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>, Result<T30, E30>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo, f29: impl FnOnce(E29) -> Eo, f30: impl FnOnce(E30) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
            match v26 {
                Ok(v) => v,
                Err(e) => Err(f26(e))?,
            },
            match v27 {
                Ok(v) => v,
                Err(e) => Err(f27(e))?,
            },
            match v28 {
                Ok(v) => v,
                Err(e) => Err(f28(e))?,
            },
            match v29 {
                Ok(v) => v,
                Err(e) => Err(f29(e))?,
            },
            match v30 {
                Ok(v) => v,
                Err(e) => Err(f30(e))?,
            },
        ))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleTransposeResultSameError for (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>, Result<T30, E>, Result<T31, E>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), E>;
    fn transpose_same_error(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?, v30?, v31?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResult32<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30, E31> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29> + From<E30> + From<E31>>(self) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29, E30, T30, E31, T31> TupleTransposeResult32<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30, E31> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>, Result<T30, E30>, Result<T31, E31>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), Eo>;
    fn transpose<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29> + From<E30> + From<E31>>(self) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?, v30?, v31?))
    }
}
#[doc = r" Transposes for Result"]
pub trait TupleTransposeResultMapErr32<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30, E31> {
    type OutTuple<Eo>;
    #[doc = r" Transposes for Result"]
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo, f29: impl FnOnce(E29) -> Eo, f30: impl FnOnce(E30) -> Eo, f31: impl FnOnce(E31) -> Eo) -> Self::OutTuple<Eo>;
}
impl<E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29, E30, T30, E31, T31> TupleTransposeResultMapErr32<E0, E1, E2, E3, E4, E5, E6, E7, E8, E9, E10, E11, E12, E13, E14, E15, E16, E17, E18, E19, E20, E21, E22, E23, E24, E25, E26, E27, E28, E29, E30, E31> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>, Result<T30, E30>, Result<T31, E31>) {
    type OutTuple<Eo> = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), Eo>;
    fn transpose_map_err<Eo>(self, f0: impl FnOnce(E0) -> Eo, f1: impl FnOnce(E1) -> Eo, f2: impl FnOnce(E2) -> Eo, f3: impl FnOnce(E3) -> Eo, f4: impl FnOnce(E4) -> Eo, f5: impl FnOnce(E5) -> Eo, f6: impl FnOnce(E6) -> Eo, f7: impl FnOnce(E7) -> Eo, f8: impl FnOnce(E8) -> Eo, f9: impl FnOnce(E9) -> Eo, f10: impl FnOnce(E10) -> Eo, f11: impl FnOnce(E11) -> Eo, f12: impl FnOnce(E12) -> Eo, f13: impl FnOnce(E13) -> Eo, f14: impl FnOnce(E14) -> Eo, f15: impl FnOnce(E15) -> Eo, f16: impl FnOnce(E16) -> Eo, f17: impl FnOnce(E17) -> Eo, f18: impl FnOnce(E18) -> Eo, f19: impl FnOnce(E19) -> Eo, f20: impl FnOnce(E20) -> Eo, f21: impl FnOnce(E21) -> Eo, f22: impl FnOnce(E22) -> Eo, f23: impl FnOnce(E23) -> Eo, f24: impl FnOnce(E24) -> Eo, f25: impl FnOnce(E25) -> Eo, f26: impl FnOnce(E26) -> Eo, f27: impl FnOnce(E27) -> Eo, f28: impl FnOnce(E28) -> Eo, f29: impl FnOnce(E29) -> Eo, f30: impl FnOnce(E30) -> Eo, f31: impl FnOnce(E31) -> Eo) -> Self::OutTuple<Eo> {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) = self;
        Ok((
            match v0 {
                Ok(v) => v,
                Err(e) => Err(f0(e))?,
            },
            match v1 {
                Ok(v) => v,
                Err(e) => Err(f1(e))?,
            },
            match v2 {
                Ok(v) => v,
                Err(e) => Err(f2(e))?,
            },
            match v3 {
                Ok(v) => v,
                Err(e) => Err(f3(e))?,
            },
            match v4 {
                Ok(v) => v,
                Err(e) => Err(f4(e))?,
            },
            match v5 {
                Ok(v) => v,
                Err(e) => Err(f5(e))?,
            },
            match v6 {
                Ok(v) => v,
                Err(e) => Err(f6(e))?,
            },
            match v7 {
                Ok(v) => v,
                Err(e) => Err(f7(e))?,
            },
            match v8 {
                Ok(v) => v,
                Err(e) => Err(f8(e))?,
            },
            match v9 {
                Ok(v) => v,
                Err(e) => Err(f9(e))?,
            },
            match v10 {
                Ok(v) => v,
                Err(e) => Err(f10(e))?,
            },
            match v11 {
                Ok(v) => v,
                Err(e) => Err(f11(e))?,
            },
            match v12 {
                Ok(v) => v,
                Err(e) => Err(f12(e))?,
            },
            match v13 {
                Ok(v) => v,
                Err(e) => Err(f13(e))?,
            },
            match v14 {
                Ok(v) => v,
                Err(e) => Err(f14(e))?,
            },
            match v15 {
                Ok(v) => v,
                Err(e) => Err(f15(e))?,
            },
            match v16 {
                Ok(v) => v,
                Err(e) => Err(f16(e))?,
            },
            match v17 {
                Ok(v) => v,
                Err(e) => Err(f17(e))?,
            },
            match v18 {
                Ok(v) => v,
                Err(e) => Err(f18(e))?,
            },
            match v19 {
                Ok(v) => v,
                Err(e) => Err(f19(e))?,
            },
            match v20 {
                Ok(v) => v,
                Err(e) => Err(f20(e))?,
            },
            match v21 {
                Ok(v) => v,
                Err(e) => Err(f21(e))?,
            },
            match v22 {
                Ok(v) => v,
                Err(e) => Err(f22(e))?,
            },
            match v23 {
                Ok(v) => v,
                Err(e) => Err(f23(e))?,
            },
            match v24 {
                Ok(v) => v,
                Err(e) => Err(f24(e))?,
            },
            match v25 {
                Ok(v) => v,
                Err(e) => Err(f25(e))?,
            },
            match v26 {
                Ok(v) => v,
                Err(e) => Err(f26(e))?,
            },
            match v27 {
                Ok(v) => v,
                Err(e) => Err(f27(e))?,
            },
            match v28 {
                Ok(v) => v,
                Err(e) => Err(f28(e))?,
            },
            match v29 {
                Ok(v) => v,
                Err(e) => Err(f29(e))?,
            },
            match v30 {
                Ok(v) => v,
                Err(e) => Err(f30(e))?,
            },
            match v31 {
                Ok(v) => v,
                Err(e) => Err(f31(e))?,
            },
        ))
    }
}
