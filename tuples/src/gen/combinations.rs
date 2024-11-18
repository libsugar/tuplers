// auto generated code, do not modify

#[doc = "Combinations by 2"]
pub trait TupleCombinations2 {
    type Output;
    #[doc = "Combinations by 2"]
    fn combinations_2(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone, T2: Clone> TupleCombinations2 for (T0, T1, T2) {
    type Output = ((T0, T1), (T0, T2), (T1, T2));
    fn combinations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0, self.2.clone()), (self.1, self.2))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone> TupleCombinations2 for (T0, T1, T2, T3) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T1, T2), (T1, T3), (T2, T3));
    fn combinations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0, self.3.clone()), (self.1.clone(), self.2.clone()), (self.1, self.3.clone()), (self.2, self.3))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone> TupleCombinations2 for (T0, T1, T2, T3, T4) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T0, T4), (T1, T2), (T1, T3), (T1, T4), (T2, T3), (T2, T4), (T3, T4));
    fn combinations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.0, self.4.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.1, self.4.clone()), (self.2.clone(), self.3.clone()), (self.2, self.4.clone()), (self.3, self.4))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TupleCombinations2 for (T0, T1, T2, T3, T4, T5) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T0, T4), (T0, T5), (T1, T2), (T1, T3), (T1, T4), (T1, T5), (T2, T3), (T2, T4), (T2, T5), (T3, T4), (T3, T5), (T4, T5));
    fn combinations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.0.clone(), self.4.clone()), (self.0, self.5.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.1.clone(), self.4.clone()), (self.1, self.5.clone()), (self.2.clone(), self.3.clone()), (self.2.clone(), self.4.clone()), (self.2, self.5.clone()), (self.3.clone(), self.4.clone()), (self.3, self.5.clone()), (self.4, self.5))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone> TupleCombinations2 for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T0, T4), (T0, T5), (T0, T6), (T1, T2), (T1, T3), (T1, T4), (T1, T5), (T1, T6), (T2, T3), (T2, T4), (T2, T5), (T2, T6), (T3, T4), (T3, T5), (T3, T6), (T4, T5), (T4, T6), (T5, T6));
    fn combinations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.0.clone(), self.4.clone()), (self.0.clone(), self.5.clone()), (self.0, self.6.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.1.clone(), self.4.clone()), (self.1.clone(), self.5.clone()), (self.1, self.6.clone()), (self.2.clone(), self.3.clone()), (self.2.clone(), self.4.clone()), (self.2.clone(), self.5.clone()), (self.2, self.6.clone()), (self.3.clone(), self.4.clone()), (self.3.clone(), self.5.clone()), (self.3, self.6.clone()), (self.4.clone(), self.5.clone()), (self.4, self.6.clone()), (self.5, self.6))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone> TupleCombinations2 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T0, T4), (T0, T5), (T0, T6), (T0, T7), (T1, T2), (T1, T3), (T1, T4), (T1, T5), (T1, T6), (T1, T7), (T2, T3), (T2, T4), (T2, T5), (T2, T6), (T2, T7), (T3, T4), (T3, T5), (T3, T6), (T3, T7), (T4, T5), (T4, T6), (T4, T7), (T5, T6), (T5, T7), (T6, T7));
    fn combinations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.0.clone(), self.4.clone()), (self.0.clone(), self.5.clone()), (self.0.clone(), self.6.clone()), (self.0, self.7.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.1.clone(), self.4.clone()), (self.1.clone(), self.5.clone()), (self.1.clone(), self.6.clone()), (self.1, self.7.clone()), (self.2.clone(), self.3.clone()), (self.2.clone(), self.4.clone()), (self.2.clone(), self.5.clone()), (self.2.clone(), self.6.clone()), (self.2, self.7.clone()), (self.3.clone(), self.4.clone()), (self.3.clone(), self.5.clone()), (self.3.clone(), self.6.clone()), (self.3, self.7.clone()), (self.4.clone(), self.5.clone()), (self.4.clone(), self.6.clone()), (self.4, self.7.clone()), (self.5.clone(), self.6.clone()), (self.5, self.7.clone()), (self.6, self.7))
    }
}
#[doc = "Combinations by 3"]
pub trait TupleCombinations3 {
    type Output;
    #[doc = "Combinations by 3"]
    fn combinations_3(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone> TupleCombinations3 for (T0, T1, T2, T3) {
    type Output = ((T0, T1, T2), (T0, T1, T3), (T0, T2, T3), (T1, T2, T3));
    fn combinations_3(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone()), (self.0.clone(), self.1.clone(), self.3.clone()), (self.0, self.2.clone(), self.3.clone()), (self.1, self.2, self.3))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone> TupleCombinations3 for (T0, T1, T2, T3, T4) {
    type Output = ((T0, T1, T2), (T0, T1, T3), (T0, T1, T4), (T0, T2, T3), (T0, T2, T4), (T0, T3, T4), (T1, T2, T3), (T1, T2, T4), (T1, T3, T4), (T2, T3, T4));
    fn combinations_3(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone()), (self.0.clone(), self.1.clone(), self.3.clone()), (self.0.clone(), self.1.clone(), self.4.clone()), (self.0.clone(), self.2.clone(), self.3.clone()), (self.0.clone(), self.2.clone(), self.4.clone()), (self.0, self.3.clone(), self.4.clone()), (self.1.clone(), self.2.clone(), self.3.clone()), (self.1.clone(), self.2.clone(), self.4.clone()), (self.1, self.3.clone(), self.4.clone()), (self.2, self.3, self.4))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TupleCombinations3 for (T0, T1, T2, T3, T4, T5) {
    type Output = ((T0, T1, T2), (T0, T1, T3), (T0, T1, T4), (T0, T1, T5), (T0, T2, T3), (T0, T2, T4), (T0, T2, T5), (T0, T3, T4), (T0, T3, T5), (T0, T4, T5), (T1, T2, T3), (T1, T2, T4), (T1, T2, T5), (T1, T3, T4), (T1, T3, T5), (T1, T4, T5), (T2, T3, T4), (T2, T3, T5), (T2, T4, T5), (T3, T4, T5));
    fn combinations_3(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone()), (self.0.clone(), self.1.clone(), self.3.clone()), (self.0.clone(), self.1.clone(), self.4.clone()), (self.0.clone(), self.1.clone(), self.5.clone()), (self.0.clone(), self.2.clone(), self.3.clone()), (self.0.clone(), self.2.clone(), self.4.clone()), (self.0.clone(), self.2.clone(), self.5.clone()), (self.0.clone(), self.3.clone(), self.4.clone()), (self.0.clone(), self.3.clone(), self.5.clone()), (self.0, self.4.clone(), self.5.clone()), (self.1.clone(), self.2.clone(), self.3.clone()), (self.1.clone(), self.2.clone(), self.4.clone()), (self.1.clone(), self.2.clone(), self.5.clone()), (self.1.clone(), self.3.clone(), self.4.clone()), (self.1.clone(), self.3.clone(), self.5.clone()), (self.1, self.4.clone(), self.5.clone()), (self.2.clone(), self.3.clone(), self.4.clone()), (self.2.clone(), self.3.clone(), self.5.clone()), (self.2, self.4.clone(), self.5.clone()), (self.3, self.4, self.5))
    }
}
#[doc = "Combinations by 4"]
pub trait TupleCombinations4 {
    type Output;
    #[doc = "Combinations by 4"]
    fn combinations_4(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone> TupleCombinations4 for (T0, T1, T2, T3, T4) {
    type Output = ((T0, T1, T2, T3), (T0, T1, T2, T4), (T0, T1, T3, T4), (T0, T2, T3, T4), (T1, T2, T3, T4));
    fn combinations_4(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone()), (self.0, self.2.clone(), self.3.clone(), self.4.clone()), (self.1, self.2, self.3, self.4))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TupleCombinations4 for (T0, T1, T2, T3, T4, T5) {
    type Output = ((T0, T1, T2, T3), (T0, T1, T2, T4), (T0, T1, T2, T5), (T0, T1, T3, T4), (T0, T1, T3, T5), (T0, T1, T4, T5), (T0, T2, T3, T4), (T0, T2, T3, T5), (T0, T2, T4, T5), (T0, T3, T4, T5), (T1, T2, T3, T4), (T1, T2, T3, T5), (T1, T2, T4, T5), (T1, T3, T4, T5), (T2, T3, T4, T5));
    fn combinations_4(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.4.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.5.clone()), (self.0.clone(), self.2.clone(), self.4.clone(), self.5.clone()), (self.0, self.3.clone(), self.4.clone(), self.5.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone()), (self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone()), (self.1, self.3.clone(), self.4.clone(), self.5.clone()), (self.2, self.3, self.4, self.5))
    }
}
#[doc = "Combinations by 5"]
pub trait TupleCombinations5 {
    type Output;
    #[doc = "Combinations by 5"]
    fn combinations_5(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TupleCombinations5 for (T0, T1, T2, T3, T4, T5) {
    type Output = ((T0, T1, T2, T3, T4), (T0, T1, T2, T3, T5), (T0, T1, T2, T4, T5), (T0, T1, T3, T4, T5), (T0, T2, T3, T4, T5), (T1, T2, T3, T4, T5));
    fn combinations_5(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.0, self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.1, self.2, self.3, self.4, self.5))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone> TupleCombinations5 for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = ((T0, T1, T2, T3, T4), (T0, T1, T2, T3, T5), (T0, T1, T2, T3, T6), (T0, T1, T2, T4, T5), (T0, T1, T2, T4, T6), (T0, T1, T2, T5, T6), (T0, T1, T3, T4, T5), (T0, T1, T3, T4, T6), (T0, T1, T3, T5, T6), (T0, T1, T4, T5, T6), (T0, T2, T3, T4, T5), (T0, T2, T3, T4, T6), (T0, T2, T3, T5, T6), (T0, T2, T4, T5, T6), (T0, T3, T4, T5, T6), (T1, T2, T3, T4, T5), (T1, T2, T3, T4, T6), (T1, T2, T3, T5, T6), (T1, T2, T4, T5, T6), (T1, T3, T4, T5, T6), (T2, T3, T4, T5, T6));
    fn combinations_5(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.6.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0, self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.6.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.6.clone()), (self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.1, self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.2, self.3, self.4, self.5, self.6))
    }
}
#[doc = "Combinations by 6"]
pub trait TupleCombinations6 {
    type Output;
    #[doc = "Combinations by 6"]
    fn combinations_6(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone> TupleCombinations6 for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = ((T0, T1, T2, T3, T4, T5), (T0, T1, T2, T3, T4, T6), (T0, T1, T2, T3, T5, T6), (T0, T1, T2, T4, T5, T6), (T0, T1, T3, T4, T5, T6), (T0, T2, T3, T4, T5, T6), (T1, T2, T3, T4, T5, T6));
    fn combinations_6(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0, self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.1, self.2, self.3, self.4, self.5, self.6))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone, T6: Clone, T7: Clone> TupleCombinations6 for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = ((T0, T1, T2, T3, T4, T5), (T0, T1, T2, T3, T4, T6), (T0, T1, T2, T3, T4, T7), (T0, T1, T2, T3, T5, T6), (T0, T1, T2, T3, T5, T7), (T0, T1, T2, T3, T6, T7), (T0, T1, T2, T4, T5, T6), (T0, T1, T2, T4, T5, T7), (T0, T1, T2, T4, T6, T7), (T0, T1, T2, T5, T6, T7), (T0, T1, T3, T4, T5, T6), (T0, T1, T3, T4, T5, T7), (T0, T1, T3, T4, T6, T7), (T0, T1, T3, T5, T6, T7), (T0, T1, T4, T5, T6, T7), (T0, T2, T3, T4, T5, T6), (T0, T2, T3, T4, T5, T7), (T0, T2, T3, T4, T6, T7), (T0, T2, T3, T5, T6, T7), (T0, T2, T4, T5, T6, T7), (T0, T3, T4, T5, T6, T7), (T1, T2, T3, T4, T5, T6), (T1, T2, T3, T4, T5, T7), (T1, T2, T3, T4, T6, T7), (T1, T2, T3, T5, T6, T7), (T1, T2, T4, T5, T6, T7), (T1, T3, T4, T5, T6, T7), (T2, T3, T4, T5, T6, T7));
    fn combinations_6(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.3.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.4.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.2.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.4.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.3.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.1.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.7.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.0.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.0, self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.5.clone(), self.7.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.4.clone(), self.6.clone(), self.7.clone()), (self.1.clone(), self.2.clone(), self.3.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.1.clone(), self.2.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.1, self.3.clone(), self.4.clone(), self.5.clone(), self.6.clone(), self.7.clone()), (self.2, self.3, self.4, self.5, self.6, self.7))
    }
}
