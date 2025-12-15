// auto generated code, do not modify

impl<'a> TupleAsRef<'a> for () {
    type Output = ();
    fn as_ref(&'a self) -> Self::Output {
        ()
    }
}
impl<'a> TupleAsMut<'a> for () {
    type Output = ();
    fn as_mut(&'a mut self) -> Self::Output {
        ()
    }
}
impl<'a> TupleAsDeref<'a> for () {
    type Output = ();
    fn as_deref(&'a self) -> Self::Output {
        ()
    }
}
impl<'a> TupleAsDerefMut<'a> for () {
    type Output = ();
    fn as_deref_mut(&'a mut self) -> Self::Output {
        ()
    }
}
impl TupleAsOption for () {
    type Output = ();
    fn as_some(self) -> Self::Output {
        ()
    }
}
impl<E> TupleAsResultOk<E> for () {
    type Output = ();
    fn as_ok(self) -> Self::Output {
        ()
    }
}
impl<O> TupleAsResultErr<O> for () {
    type Output = ();
    fn as_err(self) -> Self::Output {
        ()
    }
}
impl<U> AnyTupleAllInto<U> for () {}
impl<U> AnyTupleAllFrom<U> for () {}
impl<U> TupleAllInto<U> for () {
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U> TupleAllFrom<U> for () {
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl TupleInto<()> for () {
    fn tuple_into(self) -> () {
        ()
    }
}
impl TupleFrom<()> for () {
    fn tuple_from(src: ()) -> Self {
        ()
    }
}
impl TupleTryInto<()> for () {
    type Output = ();
    fn tuple_try_into(self) -> Self::Output {
        ()
    }
}
impl TupleTryFrom<()> for () {
    type Output = ();
    fn tuple_try_from(src: ()) -> Self::Output {
        ()
    }
}
impl<'a, T0: 'a> TupleAsRef<'a> for (T0,) {
    type Output = (&'a T0,);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0,)
    }
}
impl<'a, T0: 'a> TupleAsMut<'a> for (T0,) {
    type Output = (&'a mut T0,);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0,)
    }
}
impl<'a, T0: 'a + Deref> TupleAsDeref<'a> for (T0,) {
    type Output = (&'a <T0 as Deref>::Target,);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(),)
    }
}
impl<'a, T0: 'a + DerefMut> TupleAsDerefMut<'a> for (T0,) {
    type Output = (&'a mut <T0 as Deref>::Target,);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(),)
    }
}
impl<T0> TupleAsOption for (T0,) {
    type Output = (Option<T0>,);
    fn as_some(self) -> Self::Output {
        (Some(self.0),)
    }
}
impl<E, T0> TupleAsResultOk<E> for (T0,) {
    type Output = (Result<T0, E>,);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0),)
    }
}
impl<O, T0> TupleAsResultErr<O> for (T0,) {
    type Output = (Result<O, T0>,);
    fn as_err(self) -> Self::Output {
        (Err(self.0),)
    }
}
impl<U, T0> AnyTupleAllInto<U> for (T0,) where T0: Into<U> {}
impl<U, T0> AnyTupleAllFrom<U> for (T0,) where T0: From<U> {}
impl<U, T0> TupleAllInto<U> for (T0,)
where
    T0: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0> TupleAllFrom<U> for (T0,)
where
    T0: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, U0> TupleInto<(U0,)> for (T0,)
where
    T0: Into<U0>,
{
    fn tuple_into(self) -> (U0,) {
        (self.0.into(),)
    }
}
impl<T0, U0> TupleFrom<(U0,)> for (T0,)
where
    T0: From<U0>,
{
    fn tuple_from(src: (U0,)) -> Self {
        (src.0.into(),)
    }
}
impl<T0, U0> TupleTryInto<(U0,)> for (T0,)
where
    T0: TryInto<U0>,
{
    type Output = (Result<U0, T0::Error>,);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(),)
    }
}
impl<T0, U0> TupleTryFrom<(U0,)> for (T0,)
where
    T0: TryFrom<U0>,
{
    type Output = (Result<T0, T0::Error>,);
    fn tuple_try_from(src: (U0,)) -> Self::Output {
        (src.0.try_into(),)
    }
}
impl<'a, T0: 'a, T1: 'a> TupleAsRef<'a> for (T0, T1) {
    type Output = (&'a T0, &'a T1);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1)
    }
}
impl<'a, T0: 'a, T1: 'a> TupleAsMut<'a> for (T0, T1) {
    type Output = (&'a mut T0, &'a mut T1);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref> TupleAsDeref<'a> for (T0, T1) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut())
    }
}
impl<T0, T1> TupleAsOption for (T0, T1) {
    type Output = (Option<T0>, Option<T1>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1))
    }
}
impl<E, T0, T1> TupleAsResultOk<E> for (T0, T1) {
    type Output = (Result<T0, E>, Result<T1, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1))
    }
}
impl<O, T0, T1> TupleAsResultErr<O> for (T0, T1) {
    type Output = (Result<O, T0>, Result<O, T1>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1))
    }
}
impl<U, T0, T1> AnyTupleAllInto<U> for (T0, T1)
where
    T0: Into<U>,
    T1: Into<U>,
{
}
impl<U, T0, T1> AnyTupleAllFrom<U> for (T0, T1)
where
    T0: From<U>,
    T1: From<U>,
{
}
impl<U, T0, T1> TupleAllInto<U> for (T0, T1)
where
    T0: Into<U>,
    T1: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1> TupleAllFrom<U> for (T0, T1)
where
    T0: From<U>,
    T1: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, U0, U1> TupleInto<(U0, U1)> for (T0, T1)
where
    T0: Into<U0>,
    T1: Into<U1>,
{
    fn tuple_into(self) -> (U0, U1) {
        (self.0.into(), self.1.into())
    }
}
impl<T0, T1, U0, U1> TupleFrom<(U0, U1)> for (T0, T1)
where
    T0: From<U0>,
    T1: From<U1>,
{
    fn tuple_from(src: (U0, U1)) -> Self {
        (src.0.into(), src.1.into())
    }
}
impl<T0, T1, U0, U1> TupleTryInto<(U0, U1)> for (T0, T1)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into())
    }
}
impl<T0, T1, U0, U1> TupleTryFrom<(U0, U1)> for (T0, T1)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>);
    fn tuple_try_from(src: (U0, U1)) -> Self::Output {
        (src.0.try_into(), src.1.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a> TupleAsRef<'a> for (T0, T1, T2) {
    type Output = (&'a T0, &'a T1, &'a T2);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a> TupleAsMut<'a> for (T0, T1, T2) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut())
    }
}
impl<T0, T1, T2> TupleAsOption for (T0, T1, T2) {
    type Output = (Option<T0>, Option<T1>, Option<T2>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2))
    }
}
impl<E, T0, T1, T2> TupleAsResultOk<E> for (T0, T1, T2) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2))
    }
}
impl<O, T0, T1, T2> TupleAsResultErr<O> for (T0, T1, T2) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2))
    }
}
impl<U, T0, T1, T2> AnyTupleAllInto<U> for (T0, T1, T2)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
{
}
impl<U, T0, T1, T2> AnyTupleAllFrom<U> for (T0, T1, T2)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
{
}
impl<U, T0, T1, T2> TupleAllInto<U> for (T0, T1, T2)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2> TupleAllFrom<U> for (T0, T1, T2)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, U0, U1, U2> TupleInto<(U0, U1, U2)> for (T0, T1, T2)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
{
    fn tuple_into(self) -> (U0, U1, U2) {
        (self.0.into(), self.1.into(), self.2.into())
    }
}
impl<T0, T1, T2, U0, U1, U2> TupleFrom<(U0, U1, U2)> for (T0, T1, T2)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
{
    fn tuple_from(src: (U0, U1, U2)) -> Self {
        (src.0.into(), src.1.into(), src.2.into())
    }
}
impl<T0, T1, T2, U0, U1, U2> TupleTryInto<(U0, U1, U2)> for (T0, T1, T2)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into())
    }
}
impl<T0, T1, T2, U0, U1, U2> TupleTryFrom<(U0, U1, U2)> for (T0, T1, T2)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>);
    fn tuple_try_from(src: (U0, U1, U2)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a> TupleAsRef<'a> for (T0, T1, T2, T3) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a> TupleAsMut<'a> for (T0, T1, T2, T3) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut())
    }
}
impl<T0, T1, T2, T3> TupleAsOption for (T0, T1, T2, T3) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3))
    }
}
impl<E, T0, T1, T2, T3> TupleAsResultOk<E> for (T0, T1, T2, T3) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3))
    }
}
impl<O, T0, T1, T2, T3> TupleAsResultErr<O> for (T0, T1, T2, T3) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3))
    }
}
impl<U, T0, T1, T2, T3> AnyTupleAllInto<U> for (T0, T1, T2, T3)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
{
}
impl<U, T0, T1, T2, T3> AnyTupleAllFrom<U> for (T0, T1, T2, T3)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
{
}
impl<U, T0, T1, T2, T3> TupleAllInto<U> for (T0, T1, T2, T3)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3> TupleAllFrom<U> for (T0, T1, T2, T3)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, U0, U1, U2, U3> TupleInto<(U0, U1, U2, U3)> for (T0, T1, T2, T3)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into())
    }
}
impl<T0, T1, T2, T3, U0, U1, U2, U3> TupleFrom<(U0, U1, U2, U3)> for (T0, T1, T2, T3)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
{
    fn tuple_from(src: (U0, U1, U2, U3)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into())
    }
}
impl<T0, T1, T2, T3, U0, U1, U2, U3> TupleTryInto<(U0, U1, U2, U3)> for (T0, T1, T2, T3)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into())
    }
}
impl<T0, T1, T2, T3, U0, U1, U2, U3> TupleTryFrom<(U0, U1, U2, U3)> for (T0, T1, T2, T3)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4> TupleAsOption for (T0, T1, T2, T3, T4) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4))
    }
}
impl<E, T0, T1, T2, T3, T4> TupleAsResultOk<E> for (T0, T1, T2, T3, T4) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4))
    }
}
impl<O, T0, T1, T2, T3, T4> TupleAsResultErr<O> for (T0, T1, T2, T3, T4) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4))
    }
}
impl<U, T0, T1, T2, T3, T4> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4> TupleAllInto<U> for (T0, T1, T2, T3, T4)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4> TupleAllFrom<U> for (T0, T1, T2, T3, T4)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, U0, U1, U2, U3, U4> TupleInto<(U0, U1, U2, U3, U4)> for (T0, T1, T2, T3, T4)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into())
    }
}
impl<T0, T1, T2, T3, T4, U0, U1, U2, U3, U4> TupleFrom<(U0, U1, U2, U3, U4)> for (T0, T1, T2, T3, T4)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into())
    }
}
impl<T0, T1, T2, T3, T4, U0, U1, U2, U3, U4> TupleTryInto<(U0, U1, U2, U3, U4)> for (T0, T1, T2, T3, T4)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into())
    }
}
impl<T0, T1, T2, T3, T4, U0, U1, U2, U3, U4> TupleTryFrom<(U0, U1, U2, U3, U4)> for (T0, T1, T2, T3, T4)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleAsOption for (T0, T1, T2, T3, T4, T5) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5))
    }
}
impl<E, T0, T1, T2, T3, T4, T5> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5))
    }
}
impl<O, T0, T1, T2, T3, T4, T5> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5))
    }
}
impl<U, T0, T1, T2, T3, T4, T5> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, U0, U1, U2, U3, U4, U5> TupleInto<(U0, U1, U2, U3, U4, U5)> for (T0, T1, T2, T3, T4, T5)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, U0, U1, U2, U3, U4, U5> TupleFrom<(U0, U1, U2, U3, U4, U5)> for (T0, T1, T2, T3, T4, T5)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, U0, U1, U2, U3, U4, U5> TupleTryInto<(U0, U1, U2, U3, U4, U5)> for (T0, T1, T2, T3, T4, T5)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, U0, U1, U2, U3, U4, U5> TupleTryFrom<(U0, U1, U2, U3, U4, U5)> for (T0, T1, T2, T3, T4, T5)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, U0, U1, U2, U3, U4, U5, U6> TupleInto<(U0, U1, U2, U3, U4, U5, U6)> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, U0, U1, U2, U3, U4, U5, U6> TupleFrom<(U0, U1, U2, U3, U4, U5, U6)> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, U0, U1, U2, U3, U4, U5, U6> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6)> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, U0, U1, U2, U3, U4, U5, U6> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6)> for (T0, T1, T2, T3, T4, T5, T6)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, U0, U1, U2, U3, U4, U5, U6, U7> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7)> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, U0, U1, U2, U3, U4, U5, U6, U7> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7)> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, U0, U1, U2, U3, U4, U5, U6, U7> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7)> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, U0, U1, U2, U3, U4, U5, U6, U7> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7)> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, U0, U1, U2, U3, U4, U5, U6, U7, U8> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, U0, U1, U2, U3, U4, U5, U6, U7, U8> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, U0, U1, U2, U3, U4, U5, U6, U7, U8> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, U0, U1, U2, U3, U4, U5, U6, U7, U8> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
    T26: Into<U26>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into(), self.26.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
    T26: From<U26>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into(), src.26.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
    T26: TryInto<U26>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>, Result<U26, T26::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into(), self.26.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
    T26: TryFrom<U26>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>, Result<T26, T26::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into(), src.26.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
    T26: Into<U26>,
    T27: Into<U27>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into(), self.26.into(), self.27.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
    T26: From<U26>,
    T27: From<U27>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into(), src.26.into(), src.27.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
    T26: TryInto<U26>,
    T27: TryInto<U27>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>, Result<U26, T26::Error>, Result<U27, T27::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into(), self.26.try_into(), self.27.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
    T26: TryFrom<U26>,
    T27: TryFrom<U27>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>, Result<T26, T26::Error>, Result<T27, T27::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into(), src.26.try_into(), src.27.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
    T26: Into<U26>,
    T27: Into<U27>,
    T28: Into<U28>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into(), self.26.into(), self.27.into(), self.28.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
    T26: From<U26>,
    T27: From<U27>,
    T28: From<U28>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into(), src.26.into(), src.27.into(), src.28.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
    T26: TryInto<U26>,
    T27: TryInto<U27>,
    T28: TryInto<U28>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>, Result<U26, T26::Error>, Result<U27, T27::Error>, Result<U28, T28::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into(), self.26.try_into(), self.27.try_into(), self.28.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
    T26: TryFrom<U26>,
    T27: TryFrom<U27>,
    T28: TryFrom<U28>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>, Result<T26, T26::Error>, Result<T27, T27::Error>, Result<T28, T28::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into(), src.26.try_into(), src.27.try_into(), src.28.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28, &self.29)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28, &mut self.29)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref, T29: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target, &'a <T29 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref(), self.29.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut, T29: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target, &'a mut <T29 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut(), self.29.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28), Some(self.29))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28), Ok(self.29))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>, Result<O, T29>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28), Err(self.29))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
    T29: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
    T29: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
    T29: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
    T29: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
    T26: Into<U26>,
    T27: Into<U27>,
    T28: Into<U28>,
    T29: Into<U29>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into(), self.26.into(), self.27.into(), self.28.into(), self.29.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
    T26: From<U26>,
    T27: From<U27>,
    T28: From<U28>,
    T29: From<U29>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into(), src.26.into(), src.27.into(), src.28.into(), src.29.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
    T26: TryInto<U26>,
    T27: TryInto<U27>,
    T28: TryInto<U28>,
    T29: TryInto<U29>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>, Result<U26, T26::Error>, Result<U27, T27::Error>, Result<U28, T28::Error>, Result<U29, T29::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into(), self.26.try_into(), self.27.try_into(), self.28.try_into(), self.29.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
    T26: TryFrom<U26>,
    T27: TryFrom<U27>,
    T28: TryFrom<U28>,
    T29: TryFrom<U29>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>, Result<T26, T26::Error>, Result<T27, T27::Error>, Result<T28, T28::Error>, Result<T29, T29::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into(), src.26.try_into(), src.27.try_into(), src.28.try_into(), src.29.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28, &self.29, &self.30)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28, &mut self.29, &mut self.30)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref, T29: 'a + Deref, T30: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target, &'a <T29 as Deref>::Target, &'a <T30 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref(), self.29.deref(), self.30.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut, T29: 'a + DerefMut, T30: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target, &'a mut <T29 as Deref>::Target, &'a mut <T30 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut(), self.29.deref_mut(), self.30.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28), Some(self.29), Some(self.30))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>, Result<T30, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28), Ok(self.29), Ok(self.30))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>, Result<O, T29>, Result<O, T30>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28), Err(self.29), Err(self.30))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
    T29: Into<U>,
    T30: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
    T29: From<U>,
    T30: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
    T29: Into<U>,
    T30: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
    T29: From<U>,
    T30: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
    T26: Into<U26>,
    T27: Into<U27>,
    T28: Into<U28>,
    T29: Into<U29>,
    T30: Into<U30>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into(), self.26.into(), self.27.into(), self.28.into(), self.29.into(), self.30.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
    T26: From<U26>,
    T27: From<U27>,
    T28: From<U28>,
    T29: From<U29>,
    T30: From<U30>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into(), src.26.into(), src.27.into(), src.28.into(), src.29.into(), src.30.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
    T26: TryInto<U26>,
    T27: TryInto<U27>,
    T28: TryInto<U28>,
    T29: TryInto<U29>,
    T30: TryInto<U30>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>, Result<U26, T26::Error>, Result<U27, T27::Error>, Result<U28, T28::Error>, Result<U29, T29::Error>, Result<U30, T30::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into(), self.26.try_into(), self.27.try_into(), self.28.try_into(), self.29.try_into(), self.30.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
    T26: TryFrom<U26>,
    T27: TryFrom<U27>,
    T28: TryFrom<U28>,
    T29: TryFrom<U29>,
    T30: TryFrom<U30>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>, Result<T26, T26::Error>, Result<T27, T27::Error>, Result<T28, T28::Error>, Result<T29, T29::Error>, Result<T30, T30::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into(), src.26.try_into(), src.27.try_into(), src.28.try_into(), src.29.try_into(), src.30.try_into())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a, T31: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30, &'a T31);
    fn as_ref(&'a self) -> Self::Output {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28, &self.29, &self.30, &self.31)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a, T31: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30, &'a mut T31);
    fn as_mut(&'a mut self) -> Self::Output {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28, &mut self.29, &mut self.30, &mut self.31)
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref, T29: 'a + Deref, T30: 'a + Deref, T31: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target, &'a <T29 as Deref>::Target, &'a <T30 as Deref>::Target, &'a <T31 as Deref>::Target);
    fn as_deref(&'a self) -> Self::Output {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref(), self.29.deref(), self.30.deref(), self.31.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut, T29: 'a + DerefMut, T30: 'a + DerefMut, T31: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target, &'a mut <T29 as Deref>::Target, &'a mut <T30 as Deref>::Target, &'a mut <T31 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::Output {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut(), self.29.deref_mut(), self.30.deref_mut(), self.31.deref_mut())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>, Option<T31>);
    fn as_some(self) -> Self::Output {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28), Some(self.29), Some(self.30), Some(self.31))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>, Result<T30, E>, Result<T31, E>);
    fn as_ok(self) -> Self::Output {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28), Ok(self.29), Ok(self.30), Ok(self.31))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type Output = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>, Result<O, T29>, Result<O, T30>, Result<O, T31>);
    fn as_err(self) -> Self::Output {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28), Err(self.29), Err(self.30), Err(self.31))
    }
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> AnyTupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
    T29: Into<U>,
    T30: Into<U>,
    T31: Into<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> AnyTupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
    T29: From<U>,
    T30: From<U>,
    T31: From<U>,
{
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAllInto<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: Into<U>,
    T1: Into<U>,
    T2: Into<U>,
    T3: Into<U>,
    T4: Into<U>,
    T5: Into<U>,
    T6: Into<U>,
    T7: Into<U>,
    T8: Into<U>,
    T9: Into<U>,
    T10: Into<U>,
    T11: Into<U>,
    T12: Into<U>,
    T13: Into<U>,
    T14: Into<U>,
    T15: Into<U>,
    T16: Into<U>,
    T17: Into<U>,
    T18: Into<U>,
    T19: Into<U>,
    T20: Into<U>,
    T21: Into<U>,
    T22: Into<U>,
    T23: Into<U>,
    T24: Into<U>,
    T25: Into<U>,
    T26: Into<U>,
    T27: Into<U>,
    T28: Into<U>,
    T29: Into<U>,
    T30: Into<U>,
    T31: Into<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: Into<U>;
}
impl<U, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAllFrom<U> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: From<U>,
    T1: From<U>,
    T2: From<U>,
    T3: From<U>,
    T4: From<U>,
    T5: From<U>,
    T6: From<U>,
    T7: From<U>,
    T8: From<U>,
    T9: From<U>,
    T10: From<U>,
    T11: From<U>,
    T12: From<U>,
    T13: From<U>,
    T14: From<U>,
    T15: From<U>,
    T16: From<U>,
    T17: From<U>,
    T18: From<U>,
    T19: From<U>,
    T20: From<U>,
    T21: From<U>,
    T22: From<U>,
    T23: From<U>,
    T24: From<U>,
    T25: From<U>,
    T26: From<U>,
    T27: From<U>,
    T28: From<U>,
    T29: From<U>,
    T30: From<U>,
    T31: From<U>,
{
    type Item<const N: usize>
        = <Self as TupleItem<N>>::ItemN
    where
        Self: TupleItem<N>,
        <Self as TupleItem<N>>::ItemN: From<U>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31> TupleInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: Into<U0>,
    T1: Into<U1>,
    T2: Into<U2>,
    T3: Into<U3>,
    T4: Into<U4>,
    T5: Into<U5>,
    T6: Into<U6>,
    T7: Into<U7>,
    T8: Into<U8>,
    T9: Into<U9>,
    T10: Into<U10>,
    T11: Into<U11>,
    T12: Into<U12>,
    T13: Into<U13>,
    T14: Into<U14>,
    T15: Into<U15>,
    T16: Into<U16>,
    T17: Into<U17>,
    T18: Into<U18>,
    T19: Into<U19>,
    T20: Into<U20>,
    T21: Into<U21>,
    T22: Into<U22>,
    T23: Into<U23>,
    T24: Into<U24>,
    T25: Into<U25>,
    T26: Into<U26>,
    T27: Into<U27>,
    T28: Into<U28>,
    T29: Into<U29>,
    T30: Into<U30>,
    T31: Into<U31>,
{
    fn tuple_into(self) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31) {
        (self.0.into(), self.1.into(), self.2.into(), self.3.into(), self.4.into(), self.5.into(), self.6.into(), self.7.into(), self.8.into(), self.9.into(), self.10.into(), self.11.into(), self.12.into(), self.13.into(), self.14.into(), self.15.into(), self.16.into(), self.17.into(), self.18.into(), self.19.into(), self.20.into(), self.21.into(), self.22.into(), self.23.into(), self.24.into(), self.25.into(), self.26.into(), self.27.into(), self.28.into(), self.29.into(), self.30.into(), self.31.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31> TupleFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: From<U0>,
    T1: From<U1>,
    T2: From<U2>,
    T3: From<U3>,
    T4: From<U4>,
    T5: From<U5>,
    T6: From<U6>,
    T7: From<U7>,
    T8: From<U8>,
    T9: From<U9>,
    T10: From<U10>,
    T11: From<U11>,
    T12: From<U12>,
    T13: From<U13>,
    T14: From<U14>,
    T15: From<U15>,
    T16: From<U16>,
    T17: From<U17>,
    T18: From<U18>,
    T19: From<U19>,
    T20: From<U20>,
    T21: From<U21>,
    T22: From<U22>,
    T23: From<U23>,
    T24: From<U24>,
    T25: From<U25>,
    T26: From<U26>,
    T27: From<U27>,
    T28: From<U28>,
    T29: From<U29>,
    T30: From<U30>,
    T31: From<U31>,
{
    fn tuple_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31)) -> Self {
        (src.0.into(), src.1.into(), src.2.into(), src.3.into(), src.4.into(), src.5.into(), src.6.into(), src.7.into(), src.8.into(), src.9.into(), src.10.into(), src.11.into(), src.12.into(), src.13.into(), src.14.into(), src.15.into(), src.16.into(), src.17.into(), src.18.into(), src.19.into(), src.20.into(), src.21.into(), src.22.into(), src.23.into(), src.24.into(), src.25.into(), src.26.into(), src.27.into(), src.28.into(), src.29.into(), src.30.into(), src.31.into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31> TupleTryInto<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: TryInto<U0>,
    T1: TryInto<U1>,
    T2: TryInto<U2>,
    T3: TryInto<U3>,
    T4: TryInto<U4>,
    T5: TryInto<U5>,
    T6: TryInto<U6>,
    T7: TryInto<U7>,
    T8: TryInto<U8>,
    T9: TryInto<U9>,
    T10: TryInto<U10>,
    T11: TryInto<U11>,
    T12: TryInto<U12>,
    T13: TryInto<U13>,
    T14: TryInto<U14>,
    T15: TryInto<U15>,
    T16: TryInto<U16>,
    T17: TryInto<U17>,
    T18: TryInto<U18>,
    T19: TryInto<U19>,
    T20: TryInto<U20>,
    T21: TryInto<U21>,
    T22: TryInto<U22>,
    T23: TryInto<U23>,
    T24: TryInto<U24>,
    T25: TryInto<U25>,
    T26: TryInto<U26>,
    T27: TryInto<U27>,
    T28: TryInto<U28>,
    T29: TryInto<U29>,
    T30: TryInto<U30>,
    T31: TryInto<U31>,
{
    type Output = (Result<U0, T0::Error>, Result<U1, T1::Error>, Result<U2, T2::Error>, Result<U3, T3::Error>, Result<U4, T4::Error>, Result<U5, T5::Error>, Result<U6, T6::Error>, Result<U7, T7::Error>, Result<U8, T8::Error>, Result<U9, T9::Error>, Result<U10, T10::Error>, Result<U11, T11::Error>, Result<U12, T12::Error>, Result<U13, T13::Error>, Result<U14, T14::Error>, Result<U15, T15::Error>, Result<U16, T16::Error>, Result<U17, T17::Error>, Result<U18, T18::Error>, Result<U19, T19::Error>, Result<U20, T20::Error>, Result<U21, T21::Error>, Result<U22, T22::Error>, Result<U23, T23::Error>, Result<U24, T24::Error>, Result<U25, T25::Error>, Result<U26, T26::Error>, Result<U27, T27::Error>, Result<U28, T28::Error>, Result<U29, T29::Error>, Result<U30, T30::Error>, Result<U31, T31::Error>);
    fn tuple_try_into(self) -> Self::Output {
        (self.0.try_into(), self.1.try_into(), self.2.try_into(), self.3.try_into(), self.4.try_into(), self.5.try_into(), self.6.try_into(), self.7.try_into(), self.8.try_into(), self.9.try_into(), self.10.try_into(), self.11.try_into(), self.12.try_into(), self.13.try_into(), self.14.try_into(), self.15.try_into(), self.16.try_into(), self.17.try_into(), self.18.try_into(), self.19.try_into(), self.20.try_into(), self.21.try_into(), self.22.try_into(), self.23.try_into(), self.24.try_into(), self.25.try_into(), self.26.try_into(), self.27.try_into(), self.28.try_into(), self.29.try_into(), self.30.try_into(), self.31.try_into())
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31> TupleTryFrom<(U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    T0: TryFrom<U0>,
    T1: TryFrom<U1>,
    T2: TryFrom<U2>,
    T3: TryFrom<U3>,
    T4: TryFrom<U4>,
    T5: TryFrom<U5>,
    T6: TryFrom<U6>,
    T7: TryFrom<U7>,
    T8: TryFrom<U8>,
    T9: TryFrom<U9>,
    T10: TryFrom<U10>,
    T11: TryFrom<U11>,
    T12: TryFrom<U12>,
    T13: TryFrom<U13>,
    T14: TryFrom<U14>,
    T15: TryFrom<U15>,
    T16: TryFrom<U16>,
    T17: TryFrom<U17>,
    T18: TryFrom<U18>,
    T19: TryFrom<U19>,
    T20: TryFrom<U20>,
    T21: TryFrom<U21>,
    T22: TryFrom<U22>,
    T23: TryFrom<U23>,
    T24: TryFrom<U24>,
    T25: TryFrom<U25>,
    T26: TryFrom<U26>,
    T27: TryFrom<U27>,
    T28: TryFrom<U28>,
    T29: TryFrom<U29>,
    T30: TryFrom<U30>,
    T31: TryFrom<U31>,
{
    type Output = (Result<T0, T0::Error>, Result<T1, T1::Error>, Result<T2, T2::Error>, Result<T3, T3::Error>, Result<T4, T4::Error>, Result<T5, T5::Error>, Result<T6, T6::Error>, Result<T7, T7::Error>, Result<T8, T8::Error>, Result<T9, T9::Error>, Result<T10, T10::Error>, Result<T11, T11::Error>, Result<T12, T12::Error>, Result<T13, T13::Error>, Result<T14, T14::Error>, Result<T15, T15::Error>, Result<T16, T16::Error>, Result<T17, T17::Error>, Result<T18, T18::Error>, Result<T19, T19::Error>, Result<T20, T20::Error>, Result<T21, T21::Error>, Result<T22, T22::Error>, Result<T23, T23::Error>, Result<T24, T24::Error>, Result<T25, T25::Error>, Result<T26, T26::Error>, Result<T27, T27::Error>, Result<T28, T28::Error>, Result<T29, T29::Error>, Result<T30, T30::Error>, Result<T31, T31::Error>);
    fn tuple_try_from(src: (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31)) -> Self::Output {
        (src.0.try_into(), src.1.try_into(), src.2.try_into(), src.3.try_into(), src.4.try_into(), src.5.try_into(), src.6.try_into(), src.7.try_into(), src.8.try_into(), src.9.try_into(), src.10.try_into(), src.11.try_into(), src.12.try_into(), src.13.try_into(), src.14.try_into(), src.15.try_into(), src.16.try_into(), src.17.try_into(), src.18.try_into(), src.19.try_into(), src.20.try_into(), src.21.try_into(), src.22.try_into(), src.23.try_into(), src.24.try_into(), src.25.try_into(), src.26.try_into(), src.27.try_into(), src.28.try_into(), src.29.try_into(), src.30.try_into(), src.31.try_into())
    }
}
