// This file is by code gen, do not modify

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
impl<Eo: From<E0> + From<E1>, E0, T0, E1, T1> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>) {
    type OutTuple = Result<(T0, T1), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1) = self;
        Ok((v0?, v1?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2>, E0, T0, E1, T1, E2, T2> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>) {
    type OutTuple = Result<(T0, T1, T2), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2) = self;
        Ok((v0?, v1?, v2?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3>, E0, T0, E1, T1, E2, T2, E3, T3> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>) {
    type OutTuple = Result<(T0, T1, T2, T3), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3) = self;
        Ok((v0?, v1?, v2?, v3?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4) = self;
        Ok((v0?, v1?, v2?, v3?, v4?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29> + From<E30>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29, E30, T30> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>, Result<T30, E30>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?, v30?))
    }
}
impl<Eo: From<E0> + From<E1> + From<E2> + From<E3> + From<E4> + From<E5> + From<E6> + From<E7> + From<E8> + From<E9> + From<E10> + From<E11> + From<E12> + From<E13> + From<E14> + From<E15> + From<E16> + From<E17> + From<E18> + From<E19> + From<E20> + From<E21> + From<E22> + From<E23> + From<E24> + From<E25> + From<E26> + From<E27> + From<E28> + From<E29> + From<E30> + From<E31>, E0, T0, E1, T1, E2, T2, E3, T3, E4, T4, E5, T5, E6, T6, E7, T7, E8, T8, E9, T9, E10, T10, E11, T11, E12, T12, E13, T13, E14, T14, E15, T15, E16, T16, E17, T17, E18, T18, E19, T19, E20, T20, E21, T21, E22, T22, E23, T23, E24, T24, E25, T25, E26, T26, E27, T27, E28, T28, E29, T29, E30, T30, E31, T31> TupleTransposeResult<Eo> for (Result<T0, E0>, Result<T1, E1>, Result<T2, E2>, Result<T3, E3>, Result<T4, E4>, Result<T5, E5>, Result<T6, E6>, Result<T7, E7>, Result<T8, E8>, Result<T9, E9>, Result<T10, E10>, Result<T11, E11>, Result<T12, E12>, Result<T13, E13>, Result<T14, E14>, Result<T15, E15>, Result<T16, E16>, Result<T17, E17>, Result<T18, E18>, Result<T19, E19>, Result<T20, E20>, Result<T21, E21>, Result<T22, E22>, Result<T23, E23>, Result<T24, E24>, Result<T25, E25>, Result<T26, E26>, Result<T27, E27>, Result<T28, E28>, Result<T29, E29>, Result<T30, E30>, Result<T31, E31>) {
    type OutTuple = Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), Eo>;
    fn transpose(self) -> Self::OutTuple {
        let (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31) = self;
        Ok((v0?, v1?, v2?, v3?, v4?, v5?, v6?, v7?, v8?, v9?, v10?, v11?, v12?, v13?, v14?, v15?, v16?, v17?, v18?, v19?, v20?, v21?, v22?, v23?, v24?, v25?, v26?, v27?, v28?, v29?, v30?, v31?))
    }
}
