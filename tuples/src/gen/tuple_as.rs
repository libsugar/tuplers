// This file is by code gen, do not modify

impl<'a> TupleAsRef<'a> for () {
    type OutTuple = ();
    #[doc = "AsRef for Tuple0"]
    fn as_ref(&'a self) -> Self::OutTuple {
        ()
    }
}
impl<'a> TupleAsMut<'a> for () {
    type OutTuple = ();
    #[doc = "AsMut for Tuple0"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        ()
    }
}
impl TupleAsOption for () {
    type OutTuple = ();
    fn as_some(self) -> Self::OutTuple {
        ()
    }
}
impl<E> TupleAsResultOk<E> for () {
    type OutTuple = ();
    fn as_ok(self) -> Self::OutTuple {
        ()
    }
}
impl<O> TupleAsResultErr<O> for () {
    type OutTuple = ();
    fn as_err(self) -> Self::OutTuple {
        ()
    }
}
impl<'a> TupleAsDeref<'a> for () {
    type OutTuple = ();
    fn as_deref(&'a self) -> Self::OutTuple {
        ()
    }
}
impl<'a> TupleAsDerefMut<'a> for () {
    type OutTuple = ();
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        ()
    }
}
impl<'a, T0: 'a> TupleAsRef<'a> for (T0,) {
    type OutTuple = (&'a T0,);
    #[doc = "AsRef for Tuple1"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0,)
    }
}
impl<'a, T0: 'a> TupleAsMut<'a> for (T0,) {
    type OutTuple = (&'a mut T0,);
    #[doc = "AsMut for Tuple1"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0,)
    }
}
impl<T0> TupleAsOption for (T0,) {
    type OutTuple = (Option<T0>,);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0),)
    }
}
impl<E, T0> TupleAsResultOk<E> for (T0,) {
    type OutTuple = (Result<T0, E>,);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0),)
    }
}
impl<O, T0> TupleAsResultErr<O> for (T0,) {
    type OutTuple = (Result<O, T0>,);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0),)
    }
}
impl<'a, T0: 'a + Deref> TupleAsDeref<'a> for (T0,) {
    type OutTuple = (&'a <T0 as Deref>::Target,);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(),)
    }
}
impl<'a, T0: 'a + DerefMut> TupleAsDerefMut<'a> for (T0,) {
    type OutTuple = (&'a mut <T0 as Deref>::Target,);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(),)
    }
}
impl<'a, T0: 'a, T1: 'a> TupleAsRef<'a> for (T0, T1) {
    type OutTuple = (&'a T0, &'a T1);
    #[doc = "AsRef for Tuple2"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1)
    }
}
impl<'a, T0: 'a, T1: 'a> TupleAsMut<'a> for (T0, T1) {
    type OutTuple = (&'a mut T0, &'a mut T1);
    #[doc = "AsMut for Tuple2"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1)
    }
}
impl<T0, T1> TupleAsOption for (T0, T1) {
    type OutTuple = (Option<T0>, Option<T1>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1))
    }
}
impl<E, T0, T1> TupleAsResultOk<E> for (T0, T1) {
    type OutTuple = (Result<T0, E>, Result<T1, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1))
    }
}
impl<O, T0, T1> TupleAsResultErr<O> for (T0, T1) {
    type OutTuple = (Result<O, T0>, Result<O, T1>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref> TupleAsDeref<'a> for (T0, T1) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a> TupleAsRef<'a> for (T0, T1, T2) {
    type OutTuple = (&'a T0, &'a T1, &'a T2);
    #[doc = "AsRef for Tuple3"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a> TupleAsMut<'a> for (T0, T1, T2) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2);
    #[doc = "AsMut for Tuple3"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2)
    }
}
impl<T0, T1, T2> TupleAsOption for (T0, T1, T2) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2))
    }
}
impl<E, T0, T1, T2> TupleAsResultOk<E> for (T0, T1, T2) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2))
    }
}
impl<O, T0, T1, T2> TupleAsResultErr<O> for (T0, T1, T2) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a> TupleAsRef<'a> for (T0, T1, T2, T3) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3);
    #[doc = "AsRef for Tuple4"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a> TupleAsMut<'a> for (T0, T1, T2, T3) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3);
    #[doc = "AsMut for Tuple4"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3)
    }
}
impl<T0, T1, T2, T3> TupleAsOption for (T0, T1, T2, T3) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3))
    }
}
impl<E, T0, T1, T2, T3> TupleAsResultOk<E> for (T0, T1, T2, T3) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3))
    }
}
impl<O, T0, T1, T2, T3> TupleAsResultErr<O> for (T0, T1, T2, T3) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4);
    #[doc = "AsRef for Tuple5"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4);
    #[doc = "AsMut for Tuple5"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4)
    }
}
impl<T0, T1, T2, T3, T4> TupleAsOption for (T0, T1, T2, T3, T4) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4))
    }
}
impl<E, T0, T1, T2, T3, T4> TupleAsResultOk<E> for (T0, T1, T2, T3, T4) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4))
    }
}
impl<O, T0, T1, T2, T3, T4> TupleAsResultErr<O> for (T0, T1, T2, T3, T4) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5);
    #[doc = "AsRef for Tuple6"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5);
    #[doc = "AsMut for Tuple6"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5> TupleAsOption for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5))
    }
}
impl<E, T0, T1, T2, T3, T4, T5> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5))
    }
}
impl<O, T0, T1, T2, T3, T4, T5> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6);
    #[doc = "AsRef for Tuple7"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6);
    #[doc = "AsMut for Tuple7"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7);
    #[doc = "AsRef for Tuple8"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7);
    #[doc = "AsMut for Tuple8"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8);
    #[doc = "AsRef for Tuple9"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8);
    #[doc = "AsMut for Tuple9"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9);
    #[doc = "AsRef for Tuple10"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9);
    #[doc = "AsMut for Tuple10"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10);
    #[doc = "AsRef for Tuple11"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10);
    #[doc = "AsMut for Tuple11"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11);
    #[doc = "AsRef for Tuple12"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11);
    #[doc = "AsMut for Tuple12"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12);
    #[doc = "AsRef for Tuple13"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12);
    #[doc = "AsMut for Tuple13"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13);
    #[doc = "AsRef for Tuple14"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13);
    #[doc = "AsMut for Tuple14"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14);
    #[doc = "AsRef for Tuple15"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14);
    #[doc = "AsMut for Tuple15"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15);
    #[doc = "AsRef for Tuple16"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15);
    #[doc = "AsMut for Tuple16"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16);
    #[doc = "AsRef for Tuple17"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16);
    #[doc = "AsMut for Tuple17"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17);
    #[doc = "AsRef for Tuple18"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17);
    #[doc = "AsMut for Tuple18"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18);
    #[doc = "AsRef for Tuple19"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18);
    #[doc = "AsMut for Tuple19"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19);
    #[doc = "AsRef for Tuple20"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19);
    #[doc = "AsMut for Tuple20"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20);
    #[doc = "AsRef for Tuple21"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20);
    #[doc = "AsMut for Tuple21"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21);
    #[doc = "AsRef for Tuple22"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21);
    #[doc = "AsMut for Tuple22"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22);
    #[doc = "AsRef for Tuple23"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22);
    #[doc = "AsMut for Tuple23"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23);
    #[doc = "AsRef for Tuple24"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23);
    #[doc = "AsMut for Tuple24"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24);
    #[doc = "AsRef for Tuple25"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24);
    #[doc = "AsMut for Tuple25"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25);
    #[doc = "AsRef for Tuple26"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25);
    #[doc = "AsMut for Tuple26"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26);
    #[doc = "AsRef for Tuple27"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26);
    #[doc = "AsMut for Tuple27"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27);
    #[doc = "AsRef for Tuple28"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27);
    #[doc = "AsMut for Tuple28"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28);
    #[doc = "AsRef for Tuple29"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28);
    #[doc = "AsMut for Tuple29"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29);
    #[doc = "AsRef for Tuple30"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28, &self.29)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29);
    #[doc = "AsMut for Tuple30"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28, &mut self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28), Some(self.29))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28), Ok(self.29))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>, Result<O, T29>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28), Err(self.29))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref, T29: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target, &'a <T29 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref(), self.29.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut, T29: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target, &'a mut <T29 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut(), self.29.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30);
    #[doc = "AsRef for Tuple31"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28, &self.29, &self.30)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30);
    #[doc = "AsMut for Tuple31"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28, &mut self.29, &mut self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28), Some(self.29), Some(self.30))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>, Result<T30, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28), Ok(self.29), Ok(self.30))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>, Result<O, T29>, Result<O, T30>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28), Err(self.29), Err(self.30))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref, T29: 'a + Deref, T30: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target, &'a <T29 as Deref>::Target, &'a <T30 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref(), self.29.deref(), self.30.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut, T29: 'a + DerefMut, T30: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target, &'a mut <T29 as Deref>::Target, &'a mut <T30 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut(), self.29.deref_mut(), self.30.deref_mut())
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a, T31: 'a> TupleAsRef<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (&'a T0, &'a T1, &'a T2, &'a T3, &'a T4, &'a T5, &'a T6, &'a T7, &'a T8, &'a T9, &'a T10, &'a T11, &'a T12, &'a T13, &'a T14, &'a T15, &'a T16, &'a T17, &'a T18, &'a T19, &'a T20, &'a T21, &'a T22, &'a T23, &'a T24, &'a T25, &'a T26, &'a T27, &'a T28, &'a T29, &'a T30, &'a T31);
    #[doc = "AsRef for Tuple32"]
    fn as_ref(&'a self) -> Self::OutTuple {
        (&self.0, &self.1, &self.2, &self.3, &self.4, &self.5, &self.6, &self.7, &self.8, &self.9, &self.10, &self.11, &self.12, &self.13, &self.14, &self.15, &self.16, &self.17, &self.18, &self.19, &self.20, &self.21, &self.22, &self.23, &self.24, &self.25, &self.26, &self.27, &self.28, &self.29, &self.30, &self.31)
    }
}
impl<'a, T0: 'a, T1: 'a, T2: 'a, T3: 'a, T4: 'a, T5: 'a, T6: 'a, T7: 'a, T8: 'a, T9: 'a, T10: 'a, T11: 'a, T12: 'a, T13: 'a, T14: 'a, T15: 'a, T16: 'a, T17: 'a, T18: 'a, T19: 'a, T20: 'a, T21: 'a, T22: 'a, T23: 'a, T24: 'a, T25: 'a, T26: 'a, T27: 'a, T28: 'a, T29: 'a, T30: 'a, T31: 'a> TupleAsMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (&'a mut T0, &'a mut T1, &'a mut T2, &'a mut T3, &'a mut T4, &'a mut T5, &'a mut T6, &'a mut T7, &'a mut T8, &'a mut T9, &'a mut T10, &'a mut T11, &'a mut T12, &'a mut T13, &'a mut T14, &'a mut T15, &'a mut T16, &'a mut T17, &'a mut T18, &'a mut T19, &'a mut T20, &'a mut T21, &'a mut T22, &'a mut T23, &'a mut T24, &'a mut T25, &'a mut T26, &'a mut T27, &'a mut T28, &'a mut T29, &'a mut T30, &'a mut T31);
    #[doc = "AsMut for Tuple32"]
    fn as_mut(&'a mut self) -> Self::OutTuple {
        (&mut self.0, &mut self.1, &mut self.2, &mut self.3, &mut self.4, &mut self.5, &mut self.6, &mut self.7, &mut self.8, &mut self.9, &mut self.10, &mut self.11, &mut self.12, &mut self.13, &mut self.14, &mut self.15, &mut self.16, &mut self.17, &mut self.18, &mut self.19, &mut self.20, &mut self.21, &mut self.22, &mut self.23, &mut self.24, &mut self.25, &mut self.26, &mut self.27, &mut self.28, &mut self.29, &mut self.30, &mut self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAsOption for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (Option<T0>, Option<T1>, Option<T2>, Option<T3>, Option<T4>, Option<T5>, Option<T6>, Option<T7>, Option<T8>, Option<T9>, Option<T10>, Option<T11>, Option<T12>, Option<T13>, Option<T14>, Option<T15>, Option<T16>, Option<T17>, Option<T18>, Option<T19>, Option<T20>, Option<T21>, Option<T22>, Option<T23>, Option<T24>, Option<T25>, Option<T26>, Option<T27>, Option<T28>, Option<T29>, Option<T30>, Option<T31>);
    fn as_some(self) -> Self::OutTuple {
        (Some(self.0), Some(self.1), Some(self.2), Some(self.3), Some(self.4), Some(self.5), Some(self.6), Some(self.7), Some(self.8), Some(self.9), Some(self.10), Some(self.11), Some(self.12), Some(self.13), Some(self.14), Some(self.15), Some(self.16), Some(self.17), Some(self.18), Some(self.19), Some(self.20), Some(self.21), Some(self.22), Some(self.23), Some(self.24), Some(self.25), Some(self.26), Some(self.27), Some(self.28), Some(self.29), Some(self.30), Some(self.31))
    }
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAsResultOk<E> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (Result<T0, E>, Result<T1, E>, Result<T2, E>, Result<T3, E>, Result<T4, E>, Result<T5, E>, Result<T6, E>, Result<T7, E>, Result<T8, E>, Result<T9, E>, Result<T10, E>, Result<T11, E>, Result<T12, E>, Result<T13, E>, Result<T14, E>, Result<T15, E>, Result<T16, E>, Result<T17, E>, Result<T18, E>, Result<T19, E>, Result<T20, E>, Result<T21, E>, Result<T22, E>, Result<T23, E>, Result<T24, E>, Result<T25, E>, Result<T26, E>, Result<T27, E>, Result<T28, E>, Result<T29, E>, Result<T30, E>, Result<T31, E>);
    fn as_ok(self) -> Self::OutTuple {
        (Ok(self.0), Ok(self.1), Ok(self.2), Ok(self.3), Ok(self.4), Ok(self.5), Ok(self.6), Ok(self.7), Ok(self.8), Ok(self.9), Ok(self.10), Ok(self.11), Ok(self.12), Ok(self.13), Ok(self.14), Ok(self.15), Ok(self.16), Ok(self.17), Ok(self.18), Ok(self.19), Ok(self.20), Ok(self.21), Ok(self.22), Ok(self.23), Ok(self.24), Ok(self.25), Ok(self.26), Ok(self.27), Ok(self.28), Ok(self.29), Ok(self.30), Ok(self.31))
    }
}
impl<O, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> TupleAsResultErr<O> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (Result<O, T0>, Result<O, T1>, Result<O, T2>, Result<O, T3>, Result<O, T4>, Result<O, T5>, Result<O, T6>, Result<O, T7>, Result<O, T8>, Result<O, T9>, Result<O, T10>, Result<O, T11>, Result<O, T12>, Result<O, T13>, Result<O, T14>, Result<O, T15>, Result<O, T16>, Result<O, T17>, Result<O, T18>, Result<O, T19>, Result<O, T20>, Result<O, T21>, Result<O, T22>, Result<O, T23>, Result<O, T24>, Result<O, T25>, Result<O, T26>, Result<O, T27>, Result<O, T28>, Result<O, T29>, Result<O, T30>, Result<O, T31>);
    fn as_err(self) -> Self::OutTuple {
        (Err(self.0), Err(self.1), Err(self.2), Err(self.3), Err(self.4), Err(self.5), Err(self.6), Err(self.7), Err(self.8), Err(self.9), Err(self.10), Err(self.11), Err(self.12), Err(self.13), Err(self.14), Err(self.15), Err(self.16), Err(self.17), Err(self.18), Err(self.19), Err(self.20), Err(self.21), Err(self.22), Err(self.23), Err(self.24), Err(self.25), Err(self.26), Err(self.27), Err(self.28), Err(self.29), Err(self.30), Err(self.31))
    }
}
impl<'a, T0: 'a + Deref, T1: 'a + Deref, T2: 'a + Deref, T3: 'a + Deref, T4: 'a + Deref, T5: 'a + Deref, T6: 'a + Deref, T7: 'a + Deref, T8: 'a + Deref, T9: 'a + Deref, T10: 'a + Deref, T11: 'a + Deref, T12: 'a + Deref, T13: 'a + Deref, T14: 'a + Deref, T15: 'a + Deref, T16: 'a + Deref, T17: 'a + Deref, T18: 'a + Deref, T19: 'a + Deref, T20: 'a + Deref, T21: 'a + Deref, T22: 'a + Deref, T23: 'a + Deref, T24: 'a + Deref, T25: 'a + Deref, T26: 'a + Deref, T27: 'a + Deref, T28: 'a + Deref, T29: 'a + Deref, T30: 'a + Deref, T31: 'a + Deref> TupleAsDeref<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (&'a <T0 as Deref>::Target, &'a <T1 as Deref>::Target, &'a <T2 as Deref>::Target, &'a <T3 as Deref>::Target, &'a <T4 as Deref>::Target, &'a <T5 as Deref>::Target, &'a <T6 as Deref>::Target, &'a <T7 as Deref>::Target, &'a <T8 as Deref>::Target, &'a <T9 as Deref>::Target, &'a <T10 as Deref>::Target, &'a <T11 as Deref>::Target, &'a <T12 as Deref>::Target, &'a <T13 as Deref>::Target, &'a <T14 as Deref>::Target, &'a <T15 as Deref>::Target, &'a <T16 as Deref>::Target, &'a <T17 as Deref>::Target, &'a <T18 as Deref>::Target, &'a <T19 as Deref>::Target, &'a <T20 as Deref>::Target, &'a <T21 as Deref>::Target, &'a <T22 as Deref>::Target, &'a <T23 as Deref>::Target, &'a <T24 as Deref>::Target, &'a <T25 as Deref>::Target, &'a <T26 as Deref>::Target, &'a <T27 as Deref>::Target, &'a <T28 as Deref>::Target, &'a <T29 as Deref>::Target, &'a <T30 as Deref>::Target, &'a <T31 as Deref>::Target);
    fn as_deref(&'a self) -> Self::OutTuple {
        (self.0.deref(), self.1.deref(), self.2.deref(), self.3.deref(), self.4.deref(), self.5.deref(), self.6.deref(), self.7.deref(), self.8.deref(), self.9.deref(), self.10.deref(), self.11.deref(), self.12.deref(), self.13.deref(), self.14.deref(), self.15.deref(), self.16.deref(), self.17.deref(), self.18.deref(), self.19.deref(), self.20.deref(), self.21.deref(), self.22.deref(), self.23.deref(), self.24.deref(), self.25.deref(), self.26.deref(), self.27.deref(), self.28.deref(), self.29.deref(), self.30.deref(), self.31.deref())
    }
}
impl<'a, T0: 'a + DerefMut, T1: 'a + DerefMut, T2: 'a + DerefMut, T3: 'a + DerefMut, T4: 'a + DerefMut, T5: 'a + DerefMut, T6: 'a + DerefMut, T7: 'a + DerefMut, T8: 'a + DerefMut, T9: 'a + DerefMut, T10: 'a + DerefMut, T11: 'a + DerefMut, T12: 'a + DerefMut, T13: 'a + DerefMut, T14: 'a + DerefMut, T15: 'a + DerefMut, T16: 'a + DerefMut, T17: 'a + DerefMut, T18: 'a + DerefMut, T19: 'a + DerefMut, T20: 'a + DerefMut, T21: 'a + DerefMut, T22: 'a + DerefMut, T23: 'a + DerefMut, T24: 'a + DerefMut, T25: 'a + DerefMut, T26: 'a + DerefMut, T27: 'a + DerefMut, T28: 'a + DerefMut, T29: 'a + DerefMut, T30: 'a + DerefMut, T31: 'a + DerefMut> TupleAsDerefMut<'a> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    type OutTuple = (&'a mut <T0 as Deref>::Target, &'a mut <T1 as Deref>::Target, &'a mut <T2 as Deref>::Target, &'a mut <T3 as Deref>::Target, &'a mut <T4 as Deref>::Target, &'a mut <T5 as Deref>::Target, &'a mut <T6 as Deref>::Target, &'a mut <T7 as Deref>::Target, &'a mut <T8 as Deref>::Target, &'a mut <T9 as Deref>::Target, &'a mut <T10 as Deref>::Target, &'a mut <T11 as Deref>::Target, &'a mut <T12 as Deref>::Target, &'a mut <T13 as Deref>::Target, &'a mut <T14 as Deref>::Target, &'a mut <T15 as Deref>::Target, &'a mut <T16 as Deref>::Target, &'a mut <T17 as Deref>::Target, &'a mut <T18 as Deref>::Target, &'a mut <T19 as Deref>::Target, &'a mut <T20 as Deref>::Target, &'a mut <T21 as Deref>::Target, &'a mut <T22 as Deref>::Target, &'a mut <T23 as Deref>::Target, &'a mut <T24 as Deref>::Target, &'a mut <T25 as Deref>::Target, &'a mut <T26 as Deref>::Target, &'a mut <T27 as Deref>::Target, &'a mut <T28 as Deref>::Target, &'a mut <T29 as Deref>::Target, &'a mut <T30 as Deref>::Target, &'a mut <T31 as Deref>::Target);
    fn as_deref_mut(&'a mut self) -> Self::OutTuple {
        (self.0.deref_mut(), self.1.deref_mut(), self.2.deref_mut(), self.3.deref_mut(), self.4.deref_mut(), self.5.deref_mut(), self.6.deref_mut(), self.7.deref_mut(), self.8.deref_mut(), self.9.deref_mut(), self.10.deref_mut(), self.11.deref_mut(), self.12.deref_mut(), self.13.deref_mut(), self.14.deref_mut(), self.15.deref_mut(), self.16.deref_mut(), self.17.deref_mut(), self.18.deref_mut(), self.19.deref_mut(), self.20.deref_mut(), self.21.deref_mut(), self.22.deref_mut(), self.23.deref_mut(), self.24.deref_mut(), self.25.deref_mut(), self.26.deref_mut(), self.27.deref_mut(), self.28.deref_mut(), self.29.deref_mut(), self.30.deref_mut(), self.31.deref_mut())
    }
}
