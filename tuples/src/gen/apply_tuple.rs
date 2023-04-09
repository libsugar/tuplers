// This file is by code gen, do not modify

impl<F: FnOnce() -> R, R> ApplyTupleOnce<()> for F {
    type Output = R;
    fn apply_tuple_once(self, (): ()) -> Self::Output {
        self()
    }
}
impl<F: FnMut() -> R, R> ApplyTupleMut<()> for F {
    fn apply_tuple_mut(&mut self, (): ()) -> Self::Output {
        self()
    }
}
impl<F: Fn() -> R, R> ApplyTuple<()> for F {
    fn apply_tuple(&self, (): ()) -> Self::Output {
        self()
    }
}
impl<F: FnOnce(T0) -> R, R, T0> ApplyTupleOnce<(T0,)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0,): (T0,)) -> Self::Output {
        self(v0)
    }
}
impl<F: FnMut(T0) -> R, R, T0> ApplyTupleMut<(T0,)> for F {
    fn apply_tuple_mut(&mut self, (v0,): (T0,)) -> Self::Output {
        self(v0)
    }
}
impl<F: Fn(T0) -> R, R, T0> ApplyTuple<(T0,)> for F {
    fn apply_tuple(&self, (v0,): (T0,)) -> Self::Output {
        self(v0)
    }
}
impl<F: FnOnce(T0, T1) -> R, R, T0, T1> ApplyTupleOnce<(T0, T1)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1): (T0, T1)) -> Self::Output {
        self(v0, v1)
    }
}
impl<F: FnMut(T0, T1) -> R, R, T0, T1> ApplyTupleMut<(T0, T1)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1): (T0, T1)) -> Self::Output {
        self(v0, v1)
    }
}
impl<F: Fn(T0, T1) -> R, R, T0, T1> ApplyTuple<(T0, T1)> for F {
    fn apply_tuple(&self, (v0, v1): (T0, T1)) -> Self::Output {
        self(v0, v1)
    }
}
impl<F: FnOnce(T0, T1, T2) -> R, R, T0, T1, T2> ApplyTupleOnce<(T0, T1, T2)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2): (T0, T1, T2)) -> Self::Output {
        self(v0, v1, v2)
    }
}
impl<F: FnMut(T0, T1, T2) -> R, R, T0, T1, T2> ApplyTupleMut<(T0, T1, T2)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2): (T0, T1, T2)) -> Self::Output {
        self(v0, v1, v2)
    }
}
impl<F: Fn(T0, T1, T2) -> R, R, T0, T1, T2> ApplyTuple<(T0, T1, T2)> for F {
    fn apply_tuple(&self, (v0, v1, v2): (T0, T1, T2)) -> Self::Output {
        self(v0, v1, v2)
    }
}
impl<F: FnOnce(T0, T1, T2, T3) -> R, R, T0, T1, T2, T3> ApplyTupleOnce<(T0, T1, T2, T3)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3): (T0, T1, T2, T3)) -> Self::Output {
        self(v0, v1, v2, v3)
    }
}
impl<F: FnMut(T0, T1, T2, T3) -> R, R, T0, T1, T2, T3> ApplyTupleMut<(T0, T1, T2, T3)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3): (T0, T1, T2, T3)) -> Self::Output {
        self(v0, v1, v2, v3)
    }
}
impl<F: Fn(T0, T1, T2, T3) -> R, R, T0, T1, T2, T3> ApplyTuple<(T0, T1, T2, T3)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3): (T0, T1, T2, T3)) -> Self::Output {
        self(v0, v1, v2, v3)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4) -> R, R, T0, T1, T2, T3, T4> ApplyTupleOnce<(T0, T1, T2, T3, T4)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4): (T0, T1, T2, T3, T4)) -> Self::Output {
        self(v0, v1, v2, v3, v4)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4) -> R, R, T0, T1, T2, T3, T4> ApplyTupleMut<(T0, T1, T2, T3, T4)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4): (T0, T1, T2, T3, T4)) -> Self::Output {
        self(v0, v1, v2, v3, v4)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4) -> R, R, T0, T1, T2, T3, T4> ApplyTuple<(T0, T1, T2, T3, T4)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4): (T0, T1, T2, T3, T4)) -> Self::Output {
        self(v0, v1, v2, v3, v4)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5) -> R, R, T0, T1, T2, T3, T4, T5> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5): (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5) -> R, R, T0, T1, T2, T3, T4, T5> ApplyTupleMut<(T0, T1, T2, T3, T4, T5)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5): (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5) -> R, R, T0, T1, T2, T3, T4, T5> ApplyTuple<(T0, T1, T2, T3, T4, T5)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5): (T0, T1, T2, T3, T4, T5)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> R, R, T0, T1, T2, T3, T4, T5, T6> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6): (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6) -> R, R, T0, T1, T2, T3, T4, T5, T6> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6): (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6) -> R, R, T0, T1, T2, T3, T4, T5, T6> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6): (T0, T1, T2, T3, T4, T5, T6)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7): (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7): (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7): (T0, T1, T2, T3, T4, T5, T6, T7)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8): (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8): (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8): (T0, T1, T2, T3, T4, T5, T6, T7, T8)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
    }
}
impl<F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> ApplyTupleOnce<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)> for F {
    type Output = R;
    fn apply_tuple_once(self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
    }
}
impl<F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> ApplyTupleMut<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)> for F {
    fn apply_tuple_mut(&mut self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
    }
}
impl<F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> ApplyTuple<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)> for F {
    fn apply_tuple(&self, (v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31): (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)) -> Self::Output {
        self(v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
    }
}
