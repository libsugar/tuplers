// This file is by code gen, do not modify

impl<'a, T0: Clone, T1: Clone> TupleCloned for (&'a T0, &'a T1) {
    type TupleOut = (T0, T1);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone())
    }
}
impl<'a, T0: Clone, T1: Clone> TupleCloned for (&'a mut T0, &'a mut T1) {
    type TupleOut = (T0, T1);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone())
    }
}
impl<'a, T0: Copy, T1: Copy> TupleCopied for (&'a T0, &'a T1) {
    type TupleOut = (T0, T1);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1)
    }
}
impl<'a, T0: Copy, T1: Copy> TupleCopied for (&'a mut T0, &'a mut T1) {
    type TupleOut = (T0, T1);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2) {
    type TupleOut = (T0, T1, T2);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2) {
    type TupleOut = (T0, T1, T2);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2) {
    type TupleOut = (T0, T1, T2);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2) {
    type TupleOut = (T0, T1, T2);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3) {
    type TupleOut = (T0, T1, T2, T3);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3) {
    type TupleOut = (T0, T1, T2, T3);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3) {
    type TupleOut = (T0, T1, T2, T3);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3) {
    type TupleOut = (T0, T1, T2, T3);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4) {
    type TupleOut = (T0, T1, T2, T3, T4);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4) {
    type TupleOut = (T0, T1, T2, T3, T4);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4) {
    type TupleOut = (T0, T1, T2, T3, T4);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4) {
    type TupleOut = (T0, T1, T2, T3, T4);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5) {
    type TupleOut = (T0, T1, T2, T3, T4, T5);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5) {
    type TupleOut = (T0, T1, T2, T3, T4, T5);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5) {
    type TupleOut = (T0, T1, T2, T3, T4, T5);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5) {
    type TupleOut = (T0, T1, T2, T3, T4, T5);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone, T29: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone(), self.29.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone, T29: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone(), self.29.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy, T29: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28, *self.29)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy, T29: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28, *self.29)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone, T29: Clone, T30: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone(), self.29.clone(), self.30.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone, T29: Clone, T30: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone(), self.29.clone(), self.30.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy, T29: Copy, T30: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28, *self.29, *self.30)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy, T29: Copy, T30: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28, *self.29, *self.30)
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone, T29: Clone, T30: Clone, T31: Clone> TupleCloned for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30, &'a T31) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone(), self.29.clone(), self.30.clone(), self.31.clone())
    }
}
impl<'a, T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone, T8: Clone, T9: Clone, T10: Clone, T11: Clone, T12: Clone, T13: Clone, T14: Clone, T15: Clone, T16: Clone, T17: Clone, T18: Clone, T19: Clone, T20: Clone, T21: Clone, T22: Clone, T23: Clone, T24: Clone, T25: Clone, T26: Clone, T27: Clone, T28: Clone, T29: Clone, T30: Clone, T31: Clone> TupleCloned for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30, &'a mut T31) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn cloned(self) -> Self::TupleOut {
        (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone(), self.8.clone(), self.9.clone(), self.10.clone(), self.11.clone(), self.12.clone(), self.13.clone(), self.14.clone(), self.15.clone(), self.16.clone(), self.17.clone(), self.18.clone(), self.19.clone(), self.20.clone(), self.21.clone(), self.22.clone(), self.23.clone(), self.24.clone(), self.25.clone(), self.26.clone(), self.27.clone(), self.28.clone(), self.29.clone(), self.30.clone(), self.31.clone())
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy, T29: Copy, T30: Copy, T31: Copy> TupleCopied for (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30, &'a T31) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28, *self.29, *self.30, *self.31)
    }
}
impl<'a, T0: Copy, T1: Copy, T2: Copy, T3: Copy, T4: Copy, T5: Copy, T6: Copy, T7: Copy, T8: Copy, T9: Copy, T10: Copy, T11: Copy, T12: Copy, T13: Copy, T14: Copy, T15: Copy, T16: Copy, T17: Copy, T18: Copy, T19: Copy, T20: Copy, T21: Copy, T22: Copy, T23: Copy, T24: Copy, T25: Copy, T26: Copy, T27: Copy, T28: Copy, T29: Copy, T30: Copy, T31: Copy> TupleCopied for (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30, &'a mut T31) {
    type TupleOut = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn copied(self) -> Self::TupleOut {
        (*self.0, *self.1, *self.2, *self.3, *self.4, *self.5, *self.6, *self.7, *self.8, *self.9, *self.10, *self.11, *self.12, *self.13, *self.14, *self.15, *self.16, *self.17, *self.18, *self.19, *self.20, *self.21, *self.22, *self.23, *self.24, *self.25, *self.26, *self.27, *self.28, *self.29, *self.30, *self.31)
    }
}
