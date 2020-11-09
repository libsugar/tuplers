// This file is by code gen, do not modify

impl TupleFlatten for ((), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl TupleFlatten for ((), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()) {
    type OutTuple = ();
    fn flatten(self) -> Self::OutTuple {
        ()
    }
}
impl<T0, T1> TupleFlatten for ((T0,), (T1,)) {
    type OutTuple = (T0, T1);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0)
    }
}
impl<T0, T1, T2, T3> TupleFlatten for ((T0, T1), (T2, T3)) {
    type OutTuple = (T0, T1, T2, T3);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleFlatten for ((T0, T1, T2), (T3, T4, T5)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleFlatten for ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleFlatten for ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), (T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.0).10, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.1).10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.0).10, (self.0).11, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.1).10, (self.1).11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), (T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.0).10, (self.0).11, (self.0).12, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.1).10, (self.1).11, (self.1).12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.0).10, (self.0).11, (self.0).12, (self.0).13, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.1).10, (self.1).11, (self.1).12, (self.1).13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.0).10, (self.0).11, (self.0).12, (self.0).13, (self.0).14, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.1).10, (self.1).11, (self.1).12, (self.1).13, (self.1).14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.0).10, (self.0).11, (self.0).12, (self.0).13, (self.0).14, (self.0).15, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.1).10, (self.1).11, (self.1).12, (self.1).13, (self.1).14, (self.1).15)
    }
}
impl<T0, T1, T2> TupleFlatten for ((T0,), (T1,), (T2,)) {
    type OutTuple = (T0, T1, T2);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.2).0, (self.2).1, (self.2).2, (self.2).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleFlatten for ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleFlatten for ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.2).6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.2).6, (self.2).7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8), (T9, T10, T11, T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23, T24, T25, T26)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.2).6, (self.2).7, (self.2).8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.0).8, (self.0).9, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.1).8, (self.1).9, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.2).6, (self.2).7, (self.2).8, (self.2).9)
    }
}
impl<T0, T1, T2, T3> TupleFlatten for ((T0,), (T1,), (T2,), (T3,)) {
    type OutTuple = (T0, T1, T2, T3);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.3).0, (self.3).1, (self.3).2, (self.3).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleFlatten for ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4, (self.3).5)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6), (T7, T8, T9, T10, T11, T12, T13), (T14, T15, T16, T17, T18, T19, T20), (T21, T22, T23, T24, T25, T26, T27)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.2).6, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4, (self.3).5, (self.3).6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleFlatten for ((T0, T1, T2, T3, T4, T5, T6, T7), (T8, T9, T10, T11, T12, T13, T14, T15), (T16, T17, T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29, T30, T31)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.0).6, (self.0).7, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.1).6, (self.1).7, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.2).6, (self.2).7, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4, (self.3).5, (self.3).6, (self.3).7)
    }
}
impl<T0, T1, T2, T3, T4> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,)) {
    type OutTuple = (T0, T1, T2, T3, T4);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2, (self.4).0, (self.4).1, (self.4).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.4).0, (self.4).1, (self.4).2, (self.4).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleFlatten for ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4, (self.4).0, (self.4).1, (self.4).2, (self.4).3, (self.4).4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0, T1, T2, T3, T4, T5), (T6, T7, T8, T9, T10, T11), (T12, T13, T14, T15, T16, T17), (T18, T19, T20, T21, T22, T23), (T24, T25, T26, T27, T28, T29)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.0).5, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.1).5, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.2).5, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4, (self.3).5, (self.4).0, (self.4).1, (self.4).2, (self.4).3, (self.4).4, (self.4).5)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2, (self.4).0, (self.4).1, (self.4).2, (self.5).0, (self.5).1, (self.5).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.4).0, (self.4).1, (self.4).2, (self.4).3, (self.5).0, (self.5).1, (self.5).2, (self.5).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0, T1, T2, T3, T4), (T5, T6, T7, T8, T9), (T10, T11, T12, T13, T14), (T15, T16, T17, T18, T19), (T20, T21, T22, T23, T24), (T25, T26, T27, T28, T29)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.0).4, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.1).4, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.2).4, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.3).4, (self.4).0, (self.4).1, (self.4).2, (self.4).3, (self.4).4, (self.5).0, (self.5).1, (self.5).2, (self.5).3, (self.5).4)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2, (self.4).0, (self.4).1, (self.4).2, (self.5).0, (self.5).1, (self.5).2, (self.6).0, (self.6).1, (self.6).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.4).0, (self.4).1, (self.4).2, (self.4).3, (self.5).0, (self.5).1, (self.5).2, (self.5).3, (self.6).0, (self.6).1, (self.6).2, (self.6).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2, (self.4).0, (self.4).1, (self.4).2, (self.5).0, (self.5).1, (self.5).2, (self.6).0, (self.6).1, (self.6).2, (self.7).0, (self.7).1, (self.7).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleFlatten for ((T0, T1, T2, T3), (T4, T5, T6, T7), (T8, T9, T10, T11), (T12, T13, T14, T15), (T16, T17, T18, T19), (T20, T21, T22, T23), (T24, T25, T26, T27), (T28, T29, T30, T31)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.0).3, (self.1).0, (self.1).1, (self.1).2, (self.1).3, (self.2).0, (self.2).1, (self.2).2, (self.2).3, (self.3).0, (self.3).1, (self.3).2, (self.3).3, (self.4).0, (self.4).1, (self.4).2, (self.4).3, (self.5).0, (self.5).1, (self.5).2, (self.5).3, (self.6).0, (self.6).1, (self.6).2, (self.6).3, (self.7).0, (self.7).1, (self.7).2, (self.7).3)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2, (self.4).0, (self.4).1, (self.4).2, (self.5).0, (self.5).1, (self.5).2, (self.6).0, (self.6).1, (self.6).2, (self.7).0, (self.7).1, (self.7).2, (self.8).0, (self.8).1, (self.8).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0, T1, T2), (T3, T4, T5), (T6, T7, T8), (T9, T10, T11), (T12, T13, T14), (T15, T16, T17), (T18, T19, T20), (T21, T22, T23), (T24, T25, T26), (T27, T28, T29)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.0).2, (self.1).0, (self.1).1, (self.1).2, (self.2).0, (self.2).1, (self.2).2, (self.3).0, (self.3).1, (self.3).2, (self.4).0, (self.4).1, (self.4).2, (self.5).0, (self.5).1, (self.5).2, (self.6).0, (self.6).1, (self.6).2, (self.7).0, (self.7).1, (self.7).2, (self.8).0, (self.8).1, (self.8).2, (self.9).0, (self.9).1, (self.9).2)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1, (self.10).0, (self.10).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1, (self.10).0, (self.10).1, (self.11).0, (self.11).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1, (self.10).0, (self.10).1, (self.11).0, (self.11).1, (self.12).0, (self.12).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1, (self.10).0, (self.10).1, (self.11).0, (self.11).1, (self.12).0, (self.12).1, (self.13).0, (self.13).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1, (self.10).0, (self.10).1, (self.11).0, (self.11).1, (self.12).0, (self.12).1, (self.13).0, (self.13).1, (self.14).0, (self.14).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleFlatten for ((T0, T1), (T2, T3), (T4, T5), (T6, T7), (T8, T9), (T10, T11), (T12, T13), (T14, T15), (T16, T17), (T18, T19), (T20, T21), (T22, T23), (T24, T25), (T26, T27), (T28, T29), (T30, T31)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.0).1, (self.1).0, (self.1).1, (self.2).0, (self.2).1, (self.3).0, (self.3).1, (self.4).0, (self.4).1, (self.5).0, (self.5).1, (self.6).0, (self.6).1, (self.7).0, (self.7).1, (self.8).0, (self.8).1, (self.9).0, (self.9).1, (self.10).0, (self.10).1, (self.11).0, (self.11).1, (self.12).0, (self.12).1, (self.13).0, (self.13).1, (self.14).0, (self.14).1, (self.15).0, (self.15).1)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,), (T26,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0, (self.26).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,), (T26,), (T27,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0, (self.26).0, (self.27).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,), (T26,), (T27,), (T28,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0, (self.26).0, (self.27).0, (self.28).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,), (T26,), (T27,), (T28,), (T29,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0, (self.26).0, (self.27).0, (self.28).0, (self.29).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,), (T26,), (T27,), (T28,), (T29,), (T30,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0, (self.26).0, (self.27).0, (self.28).0, (self.29).0, (self.30).0)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleFlatten for ((T0,), (T1,), (T2,), (T3,), (T4,), (T5,), (T6,), (T7,), (T8,), (T9,), (T10,), (T11,), (T12,), (T13,), (T14,), (T15,), (T16,), (T17,), (T18,), (T19,), (T20,), (T21,), (T22,), (T23,), (T24,), (T25,), (T26,), (T27,), (T28,), (T29,), (T30,), (T31,)) {
    type OutTuple = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn flatten(self) -> Self::OutTuple {
        ((self.0).0, (self.1).0, (self.2).0, (self.3).0, (self.4).0, (self.5).0, (self.6).0, (self.7).0, (self.8).0, (self.9).0, (self.10).0, (self.11).0, (self.12).0, (self.13).0, (self.14).0, (self.15).0, (self.16).0, (self.17).0, (self.18).0, (self.19).0, (self.20).0, (self.21).0, (self.22).0, (self.23).0, (self.24).0, (self.25).0, (self.26).0, (self.27).0, (self.28).0, (self.29).0, (self.30).0, (self.31).0)
    }
}
