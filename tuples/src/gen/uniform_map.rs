// auto generated code, do not modify

impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T), U> for F {
    type Output = (U, U);
    fn apply_uniform_map(mut self, input: (T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T), U> for F {
    type Output = (U, U);
    fn apply_uniform_map(mut self, input: &(T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T), U> for F {
    type Output = (U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1))
    }
}
impl<U, T0, T1, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U> TupleUniformMapper<(T0, T1), U> for (F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: (T0, T1)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1))
    }
}
impl<U, T0, T1, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U> TupleUniformMapper<(T0, T1), U> for &mut (F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: (T0, T1)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1))
    }
}
impl<U, T0, T1, F0: Fn(T0) -> U, F1: Fn(T1) -> U> TupleUniformMapper<(T0, T1), U> for &(F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: (T0, T1)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1))
    }
}
impl<U, T0, T1, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U> TupleUniformMapper<&(T0, T1), U> for (F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: &(T0, T1)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1))
    }
}
impl<U, T0, T1, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U> TupleUniformMapper<&(T0, T1), U> for &mut (F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: &(T0, T1)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1))
    }
}
impl<U, T0, T1, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U> TupleUniformMapper<&(T0, T1), U> for &(F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: &(T0, T1)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1))
    }
}
impl<U, T0, T1, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U> TupleUniformMapper<&mut (T0, T1), U> for (F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1))
    }
}
impl<U, T0, T1, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U> TupleUniformMapper<&mut (T0, T1), U> for &mut (F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1))
    }
}
impl<U, T0, T1, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U> TupleUniformMapper<&mut (T0, T1), U> for &(F0, F1) {
    type Output = (U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T), U> for F {
    type Output = (U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T), U> for F {
    type Output = (U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T), U> for F {
    type Output = (U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2))
    }
}
impl<U, T0, T1, T2, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U> TupleUniformMapper<(T0, T1, T2), U> for (F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2))
    }
}
impl<U, T0, T1, T2, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U> TupleUniformMapper<(T0, T1, T2), U> for &mut (F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2))
    }
}
impl<U, T0, T1, T2, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U> TupleUniformMapper<(T0, T1, T2), U> for &(F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2))
    }
}
impl<U, T0, T1, T2, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U> TupleUniformMapper<&(T0, T1, T2), U> for (F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2))
    }
}
impl<U, T0, T1, T2, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U> TupleUniformMapper<&(T0, T1, T2), U> for &mut (F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2))
    }
}
impl<U, T0, T1, T2, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U> TupleUniformMapper<&(T0, T1, T2), U> for &(F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2))
    }
}
impl<U, T0, T1, T2, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U> TupleUniformMapper<&mut (T0, T1, T2), U> for (F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2))
    }
}
impl<U, T0, T1, T2, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U> TupleUniformMapper<&mut (T0, T1, T2), U> for &mut (F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2))
    }
}
impl<U, T0, T1, T2, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U> TupleUniformMapper<&mut (T0, T1, T2), U> for &(F0, F1, F2) {
    type Output = (U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T), U> for F {
    type Output = (U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T), U> for F {
    type Output = (U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T), U> for F {
    type Output = (U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U> TupleUniformMapper<(T0, T1, T2, T3), U> for (F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U> TupleUniformMapper<(T0, T1, T2, T3), U> for &mut (F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U> TupleUniformMapper<(T0, T1, T2, T3), U> for &(F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U> TupleUniformMapper<&(T0, T1, T2, T3), U> for (F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U> TupleUniformMapper<&(T0, T1, T2, T3), U> for &mut (F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U> TupleUniformMapper<&(T0, T1, T2, T3), U> for &(F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3), U> for (F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3), U> for &mut (F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3))
    }
}
impl<U, T0, T1, T2, T3, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3), U> for &(F0, F1, F2, F3) {
    type Output = (U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4), U> for (F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4), U> for &mut (F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4), U> for &(F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4), U> for (F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4), U> for &mut (F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4), U> for &(F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4), U> for (F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4), U> for &mut (F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4))
    }
}
impl<U, T0, T1, T2, T3, T4, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4), U> for &(F0, F1, F2, F3, F4) {
    type Output = (U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5), U> for (F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5), U> for &mut (F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5), U> for &(F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5), U> for (F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5), U> for &mut (F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5), U> for &(F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5), U> for (F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5), U> for &mut (F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5), U> for &(F0, F1, F2, F3, F4, F5) {
    type Output = (U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6), U> for (F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6), U> for &mut (F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6), U> for &(F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6), U> for (F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6), U> for &mut (F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6), U> for &(F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6), U> for (F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6), U> for &mut (F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6), U> for &(F0, F1, F2, F3, F4, F5, F6) {
    type Output = (U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7), U> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7), U> for &(F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7), U> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7), U> for &(F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7), U> for (F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7), U> for &(F0, F1, F2, F3, F4, F5, F6, F7) {
    type Output = (U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8) {
    type Output = (U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9) {
    type Output = (U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25), (&mut self)(input.26))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25), (&mut self)(&input.26))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25), (&mut self)(&mut input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U, F26: FnOnce(T26) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U, F26: FnMut(T26) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U, F26: Fn(T26) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U, F26: FnOnce(&T26) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U, F26: FnMut(&T26) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U, F26: Fn(&T26) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U, F26: FnOnce(&mut T26) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U, F26: FnMut(&mut T26) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U, F26: Fn(&mut T26) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25), (&mut self)(input.26), (&mut self)(input.27))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25), (&mut self)(&input.26), (&mut self)(&input.27))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25), (&mut self)(&mut input.26), (&mut self)(&mut input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U, F26: FnOnce(T26) -> U, F27: FnOnce(T27) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U, F26: FnMut(T26) -> U, F27: FnMut(T27) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U, F26: Fn(T26) -> U, F27: Fn(T27) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U, F26: FnOnce(&T26) -> U, F27: FnOnce(&T27) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U, F26: FnMut(&T26) -> U, F27: FnMut(&T27) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U, F26: Fn(&T26) -> U, F27: Fn(&T27) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U, F26: FnOnce(&mut T26) -> U, F27: FnOnce(&mut T27) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U, F26: FnMut(&mut T26) -> U, F27: FnMut(&mut T27) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U, F26: Fn(&mut T26) -> U, F27: Fn(&mut T27) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25), (&mut self)(input.26), (&mut self)(input.27), (&mut self)(input.28))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25), (&mut self)(&input.26), (&mut self)(&input.27), (&mut self)(&input.28))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25), (&mut self)(&mut input.26), (&mut self)(&mut input.27), (&mut self)(&mut input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U, F26: FnOnce(T26) -> U, F27: FnOnce(T27) -> U, F28: FnOnce(T28) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U, F26: FnMut(T26) -> U, F27: FnMut(T27) -> U, F28: FnMut(T28) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U, F26: Fn(T26) -> U, F27: Fn(T27) -> U, F28: Fn(T28) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U, F26: FnOnce(&T26) -> U, F27: FnOnce(&T27) -> U, F28: FnOnce(&T28) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U, F26: FnMut(&T26) -> U, F27: FnMut(&T27) -> U, F28: FnMut(&T28) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U, F26: Fn(&T26) -> U, F27: Fn(&T27) -> U, F28: Fn(&T28) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U, F26: FnOnce(&mut T26) -> U, F27: FnOnce(&mut T27) -> U, F28: FnOnce(&mut T28) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U, F26: FnMut(&mut T26) -> U, F27: FnMut(&mut T27) -> U, F28: FnMut(&mut T28) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U, F26: Fn(&mut T26) -> U, F27: Fn(&mut T27) -> U, F28: Fn(&mut T28) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25), (&mut self)(input.26), (&mut self)(input.27), (&mut self)(input.28), (&mut self)(input.29))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25), (&mut self)(&input.26), (&mut self)(&input.27), (&mut self)(&input.28), (&mut self)(&input.29))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25), (&mut self)(&mut input.26), (&mut self)(&mut input.27), (&mut self)(&mut input.28), (&mut self)(&mut input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U, F26: FnOnce(T26) -> U, F27: FnOnce(T27) -> U, F28: FnOnce(T28) -> U, F29: FnOnce(T29) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U, F26: FnMut(T26) -> U, F27: FnMut(T27) -> U, F28: FnMut(T28) -> U, F29: FnMut(T29) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U, F26: Fn(T26) -> U, F27: Fn(T27) -> U, F28: Fn(T28) -> U, F29: Fn(T29) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U, F26: FnOnce(&T26) -> U, F27: FnOnce(&T27) -> U, F28: FnOnce(&T28) -> U, F29: FnOnce(&T29) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U, F26: FnMut(&T26) -> U, F27: FnMut(&T27) -> U, F28: FnMut(&T28) -> U, F29: FnMut(&T29) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U, F26: Fn(&T26) -> U, F27: Fn(&T27) -> U, F28: Fn(&T28) -> U, F29: Fn(&T29) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U, F26: FnOnce(&mut T26) -> U, F27: FnOnce(&mut T27) -> U, F28: FnOnce(&mut T28) -> U, F29: FnOnce(&mut T29) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U, F26: FnMut(&mut T26) -> U, F27: FnMut(&mut T27) -> U, F28: FnMut(&mut T28) -> U, F29: FnMut(&mut T29) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U, F26: Fn(&mut T26) -> U, F27: Fn(&mut T27) -> U, F28: Fn(&mut T28) -> U, F29: Fn(&mut T29) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25), (&mut self)(input.26), (&mut self)(input.27), (&mut self)(input.28), (&mut self)(input.29), (&mut self)(input.30))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25), (&mut self)(&input.26), (&mut self)(&input.27), (&mut self)(&input.28), (&mut self)(&input.29), (&mut self)(&input.30))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25), (&mut self)(&mut input.26), (&mut self)(&mut input.27), (&mut self)(&mut input.28), (&mut self)(&mut input.29), (&mut self)(&mut input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U, F26: FnOnce(T26) -> U, F27: FnOnce(T27) -> U, F28: FnOnce(T28) -> U, F29: FnOnce(T29) -> U, F30: FnOnce(T30) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29), (self.30)(input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U, F26: FnMut(T26) -> U, F27: FnMut(T27) -> U, F28: FnMut(T28) -> U, F29: FnMut(T29) -> U, F30: FnMut(T30) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29), (self.30)(input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U, F26: Fn(T26) -> U, F27: Fn(T27) -> U, F28: Fn(T28) -> U, F29: Fn(T29) -> U, F30: Fn(T30) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29), (self.30)(input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U, F26: FnOnce(&T26) -> U, F27: FnOnce(&T27) -> U, F28: FnOnce(&T28) -> U, F29: FnOnce(&T29) -> U, F30: FnOnce(&T30) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29), (self.30)(&input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U, F26: FnMut(&T26) -> U, F27: FnMut(&T27) -> U, F28: FnMut(&T28) -> U, F29: FnMut(&T29) -> U, F30: FnMut(&T30) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29), (self.30)(&input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U, F26: Fn(&T26) -> U, F27: Fn(&T27) -> U, F28: Fn(&T28) -> U, F29: Fn(&T29) -> U, F30: Fn(&T30) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29), (self.30)(&input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U, F26: FnOnce(&mut T26) -> U, F27: FnOnce(&mut T27) -> U, F28: FnOnce(&mut T28) -> U, F29: FnOnce(&mut T29) -> U, F30: FnOnce(&mut T30) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29), (self.30)(&mut input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U, F26: FnMut(&mut T26) -> U, F27: FnMut(&mut T27) -> U, F28: FnMut(&mut T28) -> U, F29: FnMut(&mut T29) -> U, F30: FnMut(&mut T30) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29), (self.30)(&mut input.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U, F26: Fn(&mut T26) -> U, F27: Fn(&mut T27) -> U, F28: Fn(&mut T28) -> U, F29: Fn(&mut T29) -> U, F30: Fn(&mut T30) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29), (self.30)(&mut input.30))
    }
}
impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(input.0), (&mut self)(input.1), (&mut self)(input.2), (&mut self)(input.3), (&mut self)(input.4), (&mut self)(input.5), (&mut self)(input.6), (&mut self)(input.7), (&mut self)(input.8), (&mut self)(input.9), (&mut self)(input.10), (&mut self)(input.11), (&mut self)(input.12), (&mut self)(input.13), (&mut self)(input.14), (&mut self)(input.15), (&mut self)(input.16), (&mut self)(input.17), (&mut self)(input.18), (&mut self)(input.19), (&mut self)(input.20), (&mut self)(input.21), (&mut self)(input.22), (&mut self)(input.23), (&mut self)(input.24), (&mut self)(input.25), (&mut self)(input.26), (&mut self)(input.27), (&mut self)(input.28), (&mut self)(input.29), (&mut self)(input.30), (&mut self)(input.31))
    }
}
impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &(T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&input.0), (&mut self)(&input.1), (&mut self)(&input.2), (&mut self)(&input.3), (&mut self)(&input.4), (&mut self)(&input.5), (&mut self)(&input.6), (&mut self)(&input.7), (&mut self)(&input.8), (&mut self)(&input.9), (&mut self)(&input.10), (&mut self)(&input.11), (&mut self)(&input.12), (&mut self)(&input.13), (&mut self)(&input.14), (&mut self)(&input.15), (&mut self)(&input.16), (&mut self)(&input.17), (&mut self)(&input.18), (&mut self)(&input.19), (&mut self)(&input.20), (&mut self)(&input.21), (&mut self)(&input.22), (&mut self)(&input.23), (&mut self)(&input.24), (&mut self)(&input.25), (&mut self)(&input.26), (&mut self)(&input.27), (&mut self)(&input.28), (&mut self)(&input.29), (&mut self)(&input.30), (&mut self)(&input.31))
    }
}
impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T), U> for F {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(mut self, input: &mut (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T)) -> Self::Output {
        ((&mut self)(&mut input.0), (&mut self)(&mut input.1), (&mut self)(&mut input.2), (&mut self)(&mut input.3), (&mut self)(&mut input.4), (&mut self)(&mut input.5), (&mut self)(&mut input.6), (&mut self)(&mut input.7), (&mut self)(&mut input.8), (&mut self)(&mut input.9), (&mut self)(&mut input.10), (&mut self)(&mut input.11), (&mut self)(&mut input.12), (&mut self)(&mut input.13), (&mut self)(&mut input.14), (&mut self)(&mut input.15), (&mut self)(&mut input.16), (&mut self)(&mut input.17), (&mut self)(&mut input.18), (&mut self)(&mut input.19), (&mut self)(&mut input.20), (&mut self)(&mut input.21), (&mut self)(&mut input.22), (&mut self)(&mut input.23), (&mut self)(&mut input.24), (&mut self)(&mut input.25), (&mut self)(&mut input.26), (&mut self)(&mut input.27), (&mut self)(&mut input.28), (&mut self)(&mut input.29), (&mut self)(&mut input.30), (&mut self)(&mut input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: FnOnce(T0) -> U, F1: FnOnce(T1) -> U, F2: FnOnce(T2) -> U, F3: FnOnce(T3) -> U, F4: FnOnce(T4) -> U, F5: FnOnce(T5) -> U, F6: FnOnce(T6) -> U, F7: FnOnce(T7) -> U, F8: FnOnce(T8) -> U, F9: FnOnce(T9) -> U, F10: FnOnce(T10) -> U, F11: FnOnce(T11) -> U, F12: FnOnce(T12) -> U, F13: FnOnce(T13) -> U, F14: FnOnce(T14) -> U, F15: FnOnce(T15) -> U, F16: FnOnce(T16) -> U, F17: FnOnce(T17) -> U, F18: FnOnce(T18) -> U, F19: FnOnce(T19) -> U, F20: FnOnce(T20) -> U, F21: FnOnce(T21) -> U, F22: FnOnce(T22) -> U, F23: FnOnce(T23) -> U, F24: FnOnce(T24) -> U, F25: FnOnce(T25) -> U, F26: FnOnce(T26) -> U, F27: FnOnce(T27) -> U, F28: FnOnce(T28) -> U, F29: FnOnce(T29) -> U, F30: FnOnce(T30) -> U, F31: FnOnce(T31) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29), (self.30)(input.30), (self.31)(input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: FnMut(T0) -> U, F1: FnMut(T1) -> U, F2: FnMut(T2) -> U, F3: FnMut(T3) -> U, F4: FnMut(T4) -> U, F5: FnMut(T5) -> U, F6: FnMut(T6) -> U, F7: FnMut(T7) -> U, F8: FnMut(T8) -> U, F9: FnMut(T9) -> U, F10: FnMut(T10) -> U, F11: FnMut(T11) -> U, F12: FnMut(T12) -> U, F13: FnMut(T13) -> U, F14: FnMut(T14) -> U, F15: FnMut(T15) -> U, F16: FnMut(T16) -> U, F17: FnMut(T17) -> U, F18: FnMut(T18) -> U, F19: FnMut(T19) -> U, F20: FnMut(T20) -> U, F21: FnMut(T21) -> U, F22: FnMut(T22) -> U, F23: FnMut(T23) -> U, F24: FnMut(T24) -> U, F25: FnMut(T25) -> U, F26: FnMut(T26) -> U, F27: FnMut(T27) -> U, F28: FnMut(T28) -> U, F29: FnMut(T29) -> U, F30: FnMut(T30) -> U, F31: FnMut(T31) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29), (self.30)(input.30), (self.31)(input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: Fn(T0) -> U, F1: Fn(T1) -> U, F2: Fn(T2) -> U, F3: Fn(T3) -> U, F4: Fn(T4) -> U, F5: Fn(T5) -> U, F6: Fn(T6) -> U, F7: Fn(T7) -> U, F8: Fn(T8) -> U, F9: Fn(T9) -> U, F10: Fn(T10) -> U, F11: Fn(T11) -> U, F12: Fn(T12) -> U, F13: Fn(T13) -> U, F14: Fn(T14) -> U, F15: Fn(T15) -> U, F16: Fn(T16) -> U, F17: Fn(T17) -> U, F18: Fn(T18) -> U, F19: Fn(T19) -> U, F20: Fn(T20) -> U, F21: Fn(T21) -> U, F22: Fn(T22) -> U, F23: Fn(T23) -> U, F24: Fn(T24) -> U, F25: Fn(T25) -> U, F26: Fn(T26) -> U, F27: Fn(T27) -> U, F28: Fn(T28) -> U, F29: Fn(T29) -> U, F30: Fn(T30) -> U, F31: Fn(T31) -> U> TupleUniformMapper<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(input.0), (self.1)(input.1), (self.2)(input.2), (self.3)(input.3), (self.4)(input.4), (self.5)(input.5), (self.6)(input.6), (self.7)(input.7), (self.8)(input.8), (self.9)(input.9), (self.10)(input.10), (self.11)(input.11), (self.12)(input.12), (self.13)(input.13), (self.14)(input.14), (self.15)(input.15), (self.16)(input.16), (self.17)(input.17), (self.18)(input.18), (self.19)(input.19), (self.20)(input.20), (self.21)(input.21), (self.22)(input.22), (self.23)(input.23), (self.24)(input.24), (self.25)(input.25), (self.26)(input.26), (self.27)(input.27), (self.28)(input.28), (self.29)(input.29), (self.30)(input.30), (self.31)(input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: FnOnce(&T0) -> U, F1: FnOnce(&T1) -> U, F2: FnOnce(&T2) -> U, F3: FnOnce(&T3) -> U, F4: FnOnce(&T4) -> U, F5: FnOnce(&T5) -> U, F6: FnOnce(&T6) -> U, F7: FnOnce(&T7) -> U, F8: FnOnce(&T8) -> U, F9: FnOnce(&T9) -> U, F10: FnOnce(&T10) -> U, F11: FnOnce(&T11) -> U, F12: FnOnce(&T12) -> U, F13: FnOnce(&T13) -> U, F14: FnOnce(&T14) -> U, F15: FnOnce(&T15) -> U, F16: FnOnce(&T16) -> U, F17: FnOnce(&T17) -> U, F18: FnOnce(&T18) -> U, F19: FnOnce(&T19) -> U, F20: FnOnce(&T20) -> U, F21: FnOnce(&T21) -> U, F22: FnOnce(&T22) -> U, F23: FnOnce(&T23) -> U, F24: FnOnce(&T24) -> U, F25: FnOnce(&T25) -> U, F26: FnOnce(&T26) -> U, F27: FnOnce(&T27) -> U, F28: FnOnce(&T28) -> U, F29: FnOnce(&T29) -> U, F30: FnOnce(&T30) -> U, F31: FnOnce(&T31) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29), (self.30)(&input.30), (self.31)(&input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: FnMut(&T0) -> U, F1: FnMut(&T1) -> U, F2: FnMut(&T2) -> U, F3: FnMut(&T3) -> U, F4: FnMut(&T4) -> U, F5: FnMut(&T5) -> U, F6: FnMut(&T6) -> U, F7: FnMut(&T7) -> U, F8: FnMut(&T8) -> U, F9: FnMut(&T9) -> U, F10: FnMut(&T10) -> U, F11: FnMut(&T11) -> U, F12: FnMut(&T12) -> U, F13: FnMut(&T13) -> U, F14: FnMut(&T14) -> U, F15: FnMut(&T15) -> U, F16: FnMut(&T16) -> U, F17: FnMut(&T17) -> U, F18: FnMut(&T18) -> U, F19: FnMut(&T19) -> U, F20: FnMut(&T20) -> U, F21: FnMut(&T21) -> U, F22: FnMut(&T22) -> U, F23: FnMut(&T23) -> U, F24: FnMut(&T24) -> U, F25: FnMut(&T25) -> U, F26: FnMut(&T26) -> U, F27: FnMut(&T27) -> U, F28: FnMut(&T28) -> U, F29: FnMut(&T29) -> U, F30: FnMut(&T30) -> U, F31: FnMut(&T31) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29), (self.30)(&input.30), (self.31)(&input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: Fn(&T0) -> U, F1: Fn(&T1) -> U, F2: Fn(&T2) -> U, F3: Fn(&T3) -> U, F4: Fn(&T4) -> U, F5: Fn(&T5) -> U, F6: Fn(&T6) -> U, F7: Fn(&T7) -> U, F8: Fn(&T8) -> U, F9: Fn(&T9) -> U, F10: Fn(&T10) -> U, F11: Fn(&T11) -> U, F12: Fn(&T12) -> U, F13: Fn(&T13) -> U, F14: Fn(&T14) -> U, F15: Fn(&T15) -> U, F16: Fn(&T16) -> U, F17: Fn(&T17) -> U, F18: Fn(&T18) -> U, F19: Fn(&T19) -> U, F20: Fn(&T20) -> U, F21: Fn(&T21) -> U, F22: Fn(&T22) -> U, F23: Fn(&T23) -> U, F24: Fn(&T24) -> U, F25: Fn(&T25) -> U, F26: Fn(&T26) -> U, F27: Fn(&T27) -> U, F28: Fn(&T28) -> U, F29: Fn(&T29) -> U, F30: Fn(&T30) -> U, F31: Fn(&T31) -> U> TupleUniformMapper<&(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(&input.0), (self.1)(&input.1), (self.2)(&input.2), (self.3)(&input.3), (self.4)(&input.4), (self.5)(&input.5), (self.6)(&input.6), (self.7)(&input.7), (self.8)(&input.8), (self.9)(&input.9), (self.10)(&input.10), (self.11)(&input.11), (self.12)(&input.12), (self.13)(&input.13), (self.14)(&input.14), (self.15)(&input.15), (self.16)(&input.16), (self.17)(&input.17), (self.18)(&input.18), (self.19)(&input.19), (self.20)(&input.20), (self.21)(&input.21), (self.22)(&input.22), (self.23)(&input.23), (self.24)(&input.24), (self.25)(&input.25), (self.26)(&input.26), (self.27)(&input.27), (self.28)(&input.28), (self.29)(&input.29), (self.30)(&input.30), (self.31)(&input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: FnOnce(&mut T0) -> U, F1: FnOnce(&mut T1) -> U, F2: FnOnce(&mut T2) -> U, F3: FnOnce(&mut T3) -> U, F4: FnOnce(&mut T4) -> U, F5: FnOnce(&mut T5) -> U, F6: FnOnce(&mut T6) -> U, F7: FnOnce(&mut T7) -> U, F8: FnOnce(&mut T8) -> U, F9: FnOnce(&mut T9) -> U, F10: FnOnce(&mut T10) -> U, F11: FnOnce(&mut T11) -> U, F12: FnOnce(&mut T12) -> U, F13: FnOnce(&mut T13) -> U, F14: FnOnce(&mut T14) -> U, F15: FnOnce(&mut T15) -> U, F16: FnOnce(&mut T16) -> U, F17: FnOnce(&mut T17) -> U, F18: FnOnce(&mut T18) -> U, F19: FnOnce(&mut T19) -> U, F20: FnOnce(&mut T20) -> U, F21: FnOnce(&mut T21) -> U, F22: FnOnce(&mut T22) -> U, F23: FnOnce(&mut T23) -> U, F24: FnOnce(&mut T24) -> U, F25: FnOnce(&mut T25) -> U, F26: FnOnce(&mut T26) -> U, F27: FnOnce(&mut T27) -> U, F28: FnOnce(&mut T28) -> U, F29: FnOnce(&mut T29) -> U, F30: FnOnce(&mut T30) -> U, F31: FnOnce(&mut T31) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29), (self.30)(&mut input.30), (self.31)(&mut input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: FnMut(&mut T0) -> U, F1: FnMut(&mut T1) -> U, F2: FnMut(&mut T2) -> U, F3: FnMut(&mut T3) -> U, F4: FnMut(&mut T4) -> U, F5: FnMut(&mut T5) -> U, F6: FnMut(&mut T6) -> U, F7: FnMut(&mut T7) -> U, F8: FnMut(&mut T8) -> U, F9: FnMut(&mut T9) -> U, F10: FnMut(&mut T10) -> U, F11: FnMut(&mut T11) -> U, F12: FnMut(&mut T12) -> U, F13: FnMut(&mut T13) -> U, F14: FnMut(&mut T14) -> U, F15: FnMut(&mut T15) -> U, F16: FnMut(&mut T16) -> U, F17: FnMut(&mut T17) -> U, F18: FnMut(&mut T18) -> U, F19: FnMut(&mut T19) -> U, F20: FnMut(&mut T20) -> U, F21: FnMut(&mut T21) -> U, F22: FnMut(&mut T22) -> U, F23: FnMut(&mut T23) -> U, F24: FnMut(&mut T24) -> U, F25: FnMut(&mut T25) -> U, F26: FnMut(&mut T26) -> U, F27: FnMut(&mut T27) -> U, F28: FnMut(&mut T28) -> U, F29: FnMut(&mut T29) -> U, F30: FnMut(&mut T30) -> U, F31: FnMut(&mut T31) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for &mut (F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29), (self.30)(&mut input.30), (self.31)(&mut input.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, F0: Fn(&mut T0) -> U, F1: Fn(&mut T1) -> U, F2: Fn(&mut T2) -> U, F3: Fn(&mut T3) -> U, F4: Fn(&mut T4) -> U, F5: Fn(&mut T5) -> U, F6: Fn(&mut T6) -> U, F7: Fn(&mut T7) -> U, F8: Fn(&mut T8) -> U, F9: Fn(&mut T9) -> U, F10: Fn(&mut T10) -> U, F11: Fn(&mut T11) -> U, F12: Fn(&mut T12) -> U, F13: Fn(&mut T13) -> U, F14: Fn(&mut T14) -> U, F15: Fn(&mut T15) -> U, F16: Fn(&mut T16) -> U, F17: Fn(&mut T17) -> U, F18: Fn(&mut T18) -> U, F19: Fn(&mut T19) -> U, F20: Fn(&mut T20) -> U, F21: Fn(&mut T21) -> U, F22: Fn(&mut T22) -> U, F23: Fn(&mut T23) -> U, F24: Fn(&mut T24) -> U, F25: Fn(&mut T25) -> U, F26: Fn(&mut T26) -> U, F27: Fn(&mut T27) -> U, F28: Fn(&mut T28) -> U, F29: Fn(&mut T29) -> U, F30: Fn(&mut T30) -> U, F31: Fn(&mut T31) -> U> TupleUniformMapper<&mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31), U> for &(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20, F21, F22, F23, F24, F25, F26, F27, F28, F29, F30, F31) {
    type Output = (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
    fn apply_uniform_map(self, input: &mut (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        ((self.0)(&mut input.0), (self.1)(&mut input.1), (self.2)(&mut input.2), (self.3)(&mut input.3), (self.4)(&mut input.4), (self.5)(&mut input.5), (self.6)(&mut input.6), (self.7)(&mut input.7), (self.8)(&mut input.8), (self.9)(&mut input.9), (self.10)(&mut input.10), (self.11)(&mut input.11), (self.12)(&mut input.12), (self.13)(&mut input.13), (self.14)(&mut input.14), (self.15)(&mut input.15), (self.16)(&mut input.16), (self.17)(&mut input.17), (self.18)(&mut input.18), (self.19)(&mut input.19), (self.20)(&mut input.20), (self.21)(&mut input.21), (self.22)(&mut input.22), (self.23)(&mut input.23), (self.24)(&mut input.24), (self.25)(&mut input.25), (self.26)(&mut input.26), (self.27)(&mut input.27), (self.28)(&mut input.28), (self.29)(&mut input.29), (self.30)(&mut input.30), (self.31)(&mut input.31))
    }
}
