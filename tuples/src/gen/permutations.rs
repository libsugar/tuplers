// auto generated code, do not modify

#[doc = "Permutation by 2"]
pub trait TuplePermutations2 {
    type Output;
    #[doc = "Permutation by 2"]
    fn permutations_2(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone> TuplePermutations2 for (T0, T1) {
    type Output = ((T0, T1), (T1, T0));
    fn permutations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.1, self.0))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone> TuplePermutations2 for (T0, T1, T2) {
    type Output = ((T0, T1), (T0, T2), (T1, T0), (T1, T2), (T2, T0), (T2, T1));
    fn permutations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.1.clone(), self.0.clone()), (self.1.clone(), self.2.clone()), (self.2.clone(), self.0), (self.2, self.1))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone> TuplePermutations2 for (T0, T1, T2, T3) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T1, T0), (T1, T2), (T1, T3), (T2, T0), (T2, T1), (T2, T3), (T3, T0), (T3, T1), (T3, T2));
    fn permutations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.1.clone(), self.0.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.2.clone(), self.0.clone()), (self.2.clone(), self.1.clone()), (self.2.clone(), self.3.clone()), (self.3.clone(), self.0), (self.3.clone(), self.1), (self.3, self.2))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone> TuplePermutations2 for (T0, T1, T2, T3, T4) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T0, T4), (T1, T0), (T1, T2), (T1, T3), (T1, T4), (T2, T0), (T2, T1), (T2, T3), (T2, T4), (T3, T0), (T3, T1), (T3, T2), (T3, T4), (T4, T0), (T4, T1), (T4, T2), (T4, T3));
    fn permutations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.0.clone(), self.4.clone()), (self.1.clone(), self.0.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.1.clone(), self.4.clone()), (self.2.clone(), self.0.clone()), (self.2.clone(), self.1.clone()), (self.2.clone(), self.3.clone()), (self.2.clone(), self.4.clone()), (self.3.clone(), self.0.clone()), (self.3.clone(), self.1.clone()), (self.3.clone(), self.2.clone()), (self.3.clone(), self.4.clone()), (self.4.clone(), self.0), (self.4.clone(), self.1), (self.4.clone(), self.2), (self.4, self.3))
    }
}
impl<T0: Clone, T1: Clone, T2: Clone, T3: Clone, T4: Clone, T5: Clone> TuplePermutations2 for (T0, T1, T2, T3, T4, T5) {
    type Output = ((T0, T1), (T0, T2), (T0, T3), (T0, T4), (T0, T5), (T1, T0), (T1, T2), (T1, T3), (T1, T4), (T1, T5), (T2, T0), (T2, T1), (T2, T3), (T2, T4), (T2, T5), (T3, T0), (T3, T1), (T3, T2), (T3, T4), (T3, T5), (T4, T0), (T4, T1), (T4, T2), (T4, T3), (T4, T5), (T5, T0), (T5, T1), (T5, T2), (T5, T3), (T5, T4));
    fn permutations_2(self) -> Self::Output {
        ((self.0.clone(), self.1.clone()), (self.0.clone(), self.2.clone()), (self.0.clone(), self.3.clone()), (self.0.clone(), self.4.clone()), (self.0.clone(), self.5.clone()), (self.1.clone(), self.0.clone()), (self.1.clone(), self.2.clone()), (self.1.clone(), self.3.clone()), (self.1.clone(), self.4.clone()), (self.1.clone(), self.5.clone()), (self.2.clone(), self.0.clone()), (self.2.clone(), self.1.clone()), (self.2.clone(), self.3.clone()), (self.2.clone(), self.4.clone()), (self.2.clone(), self.5.clone()), (self.3.clone(), self.0.clone()), (self.3.clone(), self.1.clone()), (self.3.clone(), self.2.clone()), (self.3.clone(), self.4.clone()), (self.3.clone(), self.5.clone()), (self.4.clone(), self.0.clone()), (self.4.clone(), self.1.clone()), (self.4.clone(), self.2.clone()), (self.4.clone(), self.3.clone()), (self.4.clone(), self.5.clone()), (self.5.clone(), self.0), (self.5.clone(), self.1), (self.5.clone(), self.2), (self.5.clone(), self.3), (self.5, self.4))
    }
}
#[doc = "Permutation by 3"]
pub trait TuplePermutations3 {
    type Output;
    #[doc = "Permutation by 3"]
    fn permutations_3(self) -> Self::Output;
}
impl<T0: Clone, T1: Clone, T2: Clone> TuplePermutations3 for (T0, T1, T2) {
    type Output = ((T0, T1, T2), (T0, T2, T1), (T1, T0, T2), (T1, T2, T0), (T2, T0, T1), (T2, T1, T0));
    fn permutations_3(self) -> Self::Output {
        ((self.0.clone(), self.1.clone(), self.2.clone()), (self.0.clone(), self.2.clone(), self.1.clone()), (self.1.clone(), self.0.clone(), self.2.clone()), (self.1.clone(), self.2.clone(), self.0.clone()), (self.2.clone(), self.0.clone(), self.1.clone()), (self.2, self.1, self.0))
    }
}
