// auto generated code, do not modify

impl<T0, M> TupleMapN<0, M> for (T0,)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>,);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0),)
    }
}
impl<T> TupleDynamicMap<T> for (T,) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0,) = self;
        match n {
            0 => v0 = (mapper)(v0),
            _ => return Err((v0,)),
        }
        Ok((v0,))
    }
}
impl<T0, T1, M> TupleMapN<0, M> for (T0, T1)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1)
    }
}
impl<T0, T1, M> TupleMapN<1, M> for (T0, T1)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1))
    }
}
impl<T> TupleDynamicMap<T> for (T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            _ => return Err((v0, v1)),
        }
        Ok((v0, v1))
    }
}
impl<T0, T1, M> TupleMapAll<M> for (T0, T1)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self>,
{
    type Output = (M::Output<0>, M::Output<1>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1))
    }
}
impl<T0, T1, M0, M1> TupleMapAll<(M0, M1)> for (T0, T1)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>);
    fn map_all(self, mapper: (M0, M1)) -> Self::Output {
        let (m0, m1) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1))
    }
}
impl<T0, T1, T2, M> TupleMapN<0, M> for (T0, T1, T2)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2)
    }
}
impl<T0, T1, T2, M> TupleMapN<1, M> for (T0, T1, T2)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2)
    }
}
impl<T0, T1, T2, M> TupleMapN<2, M> for (T0, T1, T2)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            _ => return Err((v0, v1, v2)),
        }
        Ok((v0, v1, v2))
    }
}
impl<T0, T1, T2, M> TupleMapAll<M> for (T0, T1, T2)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2))
    }
}
impl<T0, T1, T2, M0, M1, M2> TupleMapAll<(M0, M1, M2)> for (T0, T1, T2)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>);
    fn map_all(self, mapper: (M0, M1, M2)) -> Self::Output {
        let (m0, m1, m2) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2))
    }
}
impl<T0, T1, T2, T3, M> TupleMapN<0, M> for (T0, T1, T2, T3)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3)
    }
}
impl<T0, T1, T2, T3, M> TupleMapN<1, M> for (T0, T1, T2, T3)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3)
    }
}
impl<T0, T1, T2, T3, M> TupleMapN<2, M> for (T0, T1, T2, T3)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3)
    }
}
impl<T0, T1, T2, T3, M> TupleMapN<3, M> for (T0, T1, T2, T3)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            _ => return Err((v0, v1, v2, v3)),
        }
        Ok((v0, v1, v2, v3))
    }
}
impl<T0, T1, T2, T3, M> TupleMapAll<M> for (T0, T1, T2, T3)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3))
    }
}
impl<T0, T1, T2, T3, M0, M1, M2, M3> TupleMapAll<(M0, M1, M2, M3)> for (T0, T1, T2, T3)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>);
    fn map_all(self, mapper: (M0, M1, M2, M3)) -> Self::Output {
        let (m0, m1, m2, m3) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3))
    }
}
impl<T0, T1, T2, T3, T4, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4)
    }
}
impl<T0, T1, T2, T3, T4, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4)
    }
}
impl<T0, T1, T2, T3, T4, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4)
    }
}
impl<T0, T1, T2, T3, T4, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4)
    }
}
impl<T0, T1, T2, T3, T4, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            _ => return Err((v0, v1, v2, v3, v4)),
        }
        Ok((v0, v1, v2, v3, v4))
    }
}
impl<T0, T1, T2, T3, T4, M> TupleMapAll<M> for (T0, T1, T2, T3, T4)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4))
    }
}
impl<T0, T1, T2, T3, T4, M0, M1, M2, M3, M4> TupleMapAll<(M0, M1, M2, M3, M4)> for (T0, T1, T2, T3, T4)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4)) -> Self::Output {
        let (m0, m1, m2, m3, m4) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4))
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5)
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            _ => return Err((v0, v1, v2, v3, v4, v5)),
        }
        Ok((v0, v1, v2, v3, v4, v5))
    }
}
impl<T0, T1, T2, T3, T4, T5, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, M0, M1, M2, M3, M4, M5> TupleMapAll<(M0, M1, M2, M3, M4, M5)> for (T0, T1, T2, T3, T4, T5)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, M0, M1, M2, M3, M4, M5, M6> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6)> for (T0, T1, T2, T3, T4, T5, T6)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, M0, M1, M2, M3, M4, M5, M6, M7> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7)> for (T0, T1, T2, T3, T4, T5, T6, T7)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, M0, M1, M2, M3, M4, M5, M6, M7, M8> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25, self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>, T26);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25), self.26)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapN<26, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<26, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M::Output<26>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, mapper.do_map_once(self.26))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            26 => v26 = (mapper)(v26),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self> + TupleMapperMutN<26, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>, M::Output<26>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25), mapper.do_map_mut(self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
    M26: TupleMapperOnce<Self> + TupleMapperOnceN<26, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>, M26::Output<26>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25, m26) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25), m26.do_map_once(self.26))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25, self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>, T26, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25), self.26, self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<26, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<26, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M::Output<26>, T27);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, mapper.do_map_once(self.26), self.27)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapN<27, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<27, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M::Output<27>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, mapper.do_map_once(self.27))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            26 => v26 = (mapper)(v26),
            27 => v27 = (mapper)(v27),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self> + TupleMapperMutN<26, Self> + TupleMapperMutN<27, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>, M::Output<26>, M::Output<27>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25), mapper.do_map_mut(self.26), mapper.do_map_mut(self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
    M26: TupleMapperOnce<Self> + TupleMapperOnceN<26, Self>,
    M27: TupleMapperOnce<Self> + TupleMapperOnceN<27, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>, M26::Output<26>, M27::Output<27>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25, m26, m27) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25), m26.do_map_once(self.26), m27.do_map_once(self.27))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25, self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>, T26, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25), self.26, self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<26, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<26, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M::Output<26>, T27, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, mapper.do_map_once(self.26), self.27, self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<27, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<27, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M::Output<27>, T28);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, mapper.do_map_once(self.27), self.28)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapN<28, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<28, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M::Output<28>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, mapper.do_map_once(self.28))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            26 => v26 = (mapper)(v26),
            27 => v27 = (mapper)(v27),
            28 => v28 = (mapper)(v28),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self> + TupleMapperMutN<26, Self> + TupleMapperMutN<27, Self> + TupleMapperMutN<28, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>, M::Output<26>, M::Output<27>, M::Output<28>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25), mapper.do_map_mut(self.26), mapper.do_map_mut(self.27), mapper.do_map_mut(self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
    M26: TupleMapperOnce<Self> + TupleMapperOnceN<26, Self>,
    M27: TupleMapperOnce<Self> + TupleMapperOnceN<27, Self>,
    M28: TupleMapperOnce<Self> + TupleMapperOnceN<28, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>, M26::Output<26>, M27::Output<27>, M28::Output<28>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25, m26, m27, m28) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25), m26.do_map_once(self.26), m27.do_map_once(self.27), m28.do_map_once(self.28))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25, self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>, T26, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25), self.26, self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<26, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<26, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M::Output<26>, T27, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, mapper.do_map_once(self.26), self.27, self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<27, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<27, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M::Output<27>, T28, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, mapper.do_map_once(self.27), self.28, self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<28, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<28, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M::Output<28>, T29);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, mapper.do_map_once(self.28), self.29)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapN<29, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<29, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M::Output<29>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, mapper.do_map_once(self.29))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            26 => v26 = (mapper)(v26),
            27 => v27 = (mapper)(v27),
            28 => v28 = (mapper)(v28),
            29 => v29 = (mapper)(v29),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self> + TupleMapperMutN<26, Self> + TupleMapperMutN<27, Self> + TupleMapperMutN<28, Self> + TupleMapperMutN<29, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>, M::Output<26>, M::Output<27>, M::Output<28>, M::Output<29>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25), mapper.do_map_mut(self.26), mapper.do_map_mut(self.27), mapper.do_map_mut(self.28), mapper.do_map_mut(self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
    M26: TupleMapperOnce<Self> + TupleMapperOnceN<26, Self>,
    M27: TupleMapperOnce<Self> + TupleMapperOnceN<27, Self>,
    M28: TupleMapperOnce<Self> + TupleMapperOnceN<28, Self>,
    M29: TupleMapperOnce<Self> + TupleMapperOnceN<29, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>, M26::Output<26>, M27::Output<27>, M28::Output<28>, M29::Output<29>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25, m26, m27, m28, m29) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25), m26.do_map_once(self.26), m27.do_map_once(self.27), m28.do_map_once(self.28), m29.do_map_once(self.29))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25, self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>, T26, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25), self.26, self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<26, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<26, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M::Output<26>, T27, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, mapper.do_map_once(self.26), self.27, self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<27, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<27, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M::Output<27>, T28, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, mapper.do_map_once(self.27), self.28, self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<28, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<28, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M::Output<28>, T29, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, mapper.do_map_once(self.28), self.29, self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<29, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<29, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M::Output<29>, T30);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, mapper.do_map_once(self.29), self.30)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapN<30, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<30, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M::Output<30>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, mapper.do_map_once(self.30))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            26 => v26 = (mapper)(v26),
            27 => v27 = (mapper)(v27),
            28 => v28 = (mapper)(v28),
            29 => v29 = (mapper)(v29),
            30 => v30 = (mapper)(v30),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self> + TupleMapperMutN<26, Self> + TupleMapperMutN<27, Self> + TupleMapperMutN<28, Self> + TupleMapperMutN<29, Self> + TupleMapperMutN<30, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>, M::Output<26>, M::Output<27>, M::Output<28>, M::Output<29>, M::Output<30>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25), mapper.do_map_mut(self.26), mapper.do_map_mut(self.27), mapper.do_map_mut(self.28), mapper.do_map_mut(self.29), mapper.do_map_mut(self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
    M26: TupleMapperOnce<Self> + TupleMapperOnceN<26, Self>,
    M27: TupleMapperOnce<Self> + TupleMapperOnceN<27, Self>,
    M28: TupleMapperOnce<Self> + TupleMapperOnceN<28, Self>,
    M29: TupleMapperOnce<Self> + TupleMapperOnceN<29, Self>,
    M30: TupleMapperOnce<Self> + TupleMapperOnceN<30, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>, M26::Output<26>, M27::Output<27>, M28::Output<28>, M29::Output<29>, M30::Output<30>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25, m26, m27, m28, m29, m30) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25), m26.do_map_once(self.26), m27.do_map_once(self.27), m28.do_map_once(self.28), m29.do_map_once(self.29), m30.do_map_once(self.30))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<0, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<0, Self>,
{
    type OutputN = (M::Output<0>, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (mapper.do_map_once(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<1, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<1, Self>,
{
    type OutputN = (T0, M::Output<1>, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, mapper.do_map_once(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<2, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<2, Self>,
{
    type OutputN = (T0, T1, M::Output<2>, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, mapper.do_map_once(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<3, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<3, Self>,
{
    type OutputN = (T0, T1, T2, M::Output<3>, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, mapper.do_map_once(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<4, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<4, Self>,
{
    type OutputN = (T0, T1, T2, T3, M::Output<4>, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, mapper.do_map_once(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<5, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<5, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, M::Output<5>, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, mapper.do_map_once(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<6, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<6, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, M::Output<6>, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, mapper.do_map_once(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<7, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<7, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, M::Output<7>, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, mapper.do_map_once(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<8, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<8, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, M::Output<8>, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, mapper.do_map_once(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<9, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<9, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, M::Output<9>, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, mapper.do_map_once(self.9), self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<10, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<10, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, M::Output<10>, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, mapper.do_map_once(self.10), self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<11, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<11, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, M::Output<11>, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, mapper.do_map_once(self.11), self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<12, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<12, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, M::Output<12>, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, mapper.do_map_once(self.12), self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<13, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<13, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, M::Output<13>, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, mapper.do_map_once(self.13), self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<14, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<14, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, M::Output<14>, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, mapper.do_map_once(self.14), self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<15, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<15, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, M::Output<15>, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, mapper.do_map_once(self.15), self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<16, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<16, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, M::Output<16>, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, mapper.do_map_once(self.16), self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<17, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<17, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, M::Output<17>, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, mapper.do_map_once(self.17), self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<18, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<18, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, M::Output<18>, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, mapper.do_map_once(self.18), self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<19, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<19, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, M::Output<19>, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, mapper.do_map_once(self.19), self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<20, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<20, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, M::Output<20>, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, mapper.do_map_once(self.20), self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<21, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<21, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, M::Output<21>, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, mapper.do_map_once(self.21), self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<22, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<22, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, M::Output<22>, T23, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, mapper.do_map_once(self.22), self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<23, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<23, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, M::Output<23>, T24, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, mapper.do_map_once(self.23), self.24, self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<24, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<24, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, M::Output<24>, T25, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, mapper.do_map_once(self.24), self.25, self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<25, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<25, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, M::Output<25>, T26, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, mapper.do_map_once(self.25), self.26, self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<26, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<26, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, M::Output<26>, T27, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, mapper.do_map_once(self.26), self.27, self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<27, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<27, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, M::Output<27>, T28, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, mapper.do_map_once(self.27), self.28, self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<28, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<28, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, M::Output<28>, T29, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, mapper.do_map_once(self.28), self.29, self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<29, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<29, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, M::Output<29>, T30, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, mapper.do_map_once(self.29), self.30, self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<30, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<30, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, M::Output<30>, T31);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, mapper.do_map_once(self.30), self.31)
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapN<31, M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperOnceN<31, Self>,
{
    type OutputN = (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, M::Output<31>);
    fn map_n(self, mapper: M) -> Self::OutputN {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15, self.16, self.17, self.18, self.19, self.20, self.21, self.22, self.23, self.24, self.25, self.26, self.27, self.28, self.29, self.30, mapper.do_map_once(self.31))
    }
}
impl<T> TupleDynamicMap<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
        let (mut v0, mut v1, mut v2, mut v3, mut v4, mut v5, mut v6, mut v7, mut v8, mut v9, mut v10, mut v11, mut v12, mut v13, mut v14, mut v15, mut v16, mut v17, mut v18, mut v19, mut v20, mut v21, mut v22, mut v23, mut v24, mut v25, mut v26, mut v27, mut v28, mut v29, mut v30, mut v31) = self;
        match n {
            0 => v0 = (mapper)(v0),
            1 => v1 = (mapper)(v1),
            2 => v2 = (mapper)(v2),
            3 => v3 = (mapper)(v3),
            4 => v4 = (mapper)(v4),
            5 => v5 = (mapper)(v5),
            6 => v6 = (mapper)(v6),
            7 => v7 = (mapper)(v7),
            8 => v8 = (mapper)(v8),
            9 => v9 = (mapper)(v9),
            10 => v10 = (mapper)(v10),
            11 => v11 = (mapper)(v11),
            12 => v12 = (mapper)(v12),
            13 => v13 = (mapper)(v13),
            14 => v14 = (mapper)(v14),
            15 => v15 = (mapper)(v15),
            16 => v16 = (mapper)(v16),
            17 => v17 = (mapper)(v17),
            18 => v18 = (mapper)(v18),
            19 => v19 = (mapper)(v19),
            20 => v20 = (mapper)(v20),
            21 => v21 = (mapper)(v21),
            22 => v22 = (mapper)(v22),
            23 => v23 = (mapper)(v23),
            24 => v24 = (mapper)(v24),
            25 => v25 = (mapper)(v25),
            26 => v26 = (mapper)(v26),
            27 => v27 = (mapper)(v27),
            28 => v28 = (mapper)(v28),
            29 => v29 = (mapper)(v29),
            30 => v30 = (mapper)(v30),
            31 => v31 = (mapper)(v31),
            _ => return Err((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)),
        }
        Ok((v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M> TupleMapAll<M> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M: TupleMapperMut<Self> + TupleMapperMutN<0, Self> + TupleMapperMutN<1, Self> + TupleMapperMutN<2, Self> + TupleMapperMutN<3, Self> + TupleMapperMutN<4, Self> + TupleMapperMutN<5, Self> + TupleMapperMutN<6, Self> + TupleMapperMutN<7, Self> + TupleMapperMutN<8, Self> + TupleMapperMutN<9, Self> + TupleMapperMutN<10, Self> + TupleMapperMutN<11, Self> + TupleMapperMutN<12, Self> + TupleMapperMutN<13, Self> + TupleMapperMutN<14, Self> + TupleMapperMutN<15, Self> + TupleMapperMutN<16, Self> + TupleMapperMutN<17, Self> + TupleMapperMutN<18, Self> + TupleMapperMutN<19, Self> + TupleMapperMutN<20, Self> + TupleMapperMutN<21, Self> + TupleMapperMutN<22, Self> + TupleMapperMutN<23, Self> + TupleMapperMutN<24, Self> + TupleMapperMutN<25, Self> + TupleMapperMutN<26, Self> + TupleMapperMutN<27, Self> + TupleMapperMutN<28, Self> + TupleMapperMutN<29, Self> + TupleMapperMutN<30, Self> + TupleMapperMutN<31, Self>,
{
    type Output = (M::Output<0>, M::Output<1>, M::Output<2>, M::Output<3>, M::Output<4>, M::Output<5>, M::Output<6>, M::Output<7>, M::Output<8>, M::Output<9>, M::Output<10>, M::Output<11>, M::Output<12>, M::Output<13>, M::Output<14>, M::Output<15>, M::Output<16>, M::Output<17>, M::Output<18>, M::Output<19>, M::Output<20>, M::Output<21>, M::Output<22>, M::Output<23>, M::Output<24>, M::Output<25>, M::Output<26>, M::Output<27>, M::Output<28>, M::Output<29>, M::Output<30>, M::Output<31>);
    fn map_all(self, mut mapper: M) -> Self::Output {
        (mapper.do_map_mut(self.0), mapper.do_map_mut(self.1), mapper.do_map_mut(self.2), mapper.do_map_mut(self.3), mapper.do_map_mut(self.4), mapper.do_map_mut(self.5), mapper.do_map_mut(self.6), mapper.do_map_mut(self.7), mapper.do_map_mut(self.8), mapper.do_map_mut(self.9), mapper.do_map_mut(self.10), mapper.do_map_mut(self.11), mapper.do_map_mut(self.12), mapper.do_map_mut(self.13), mapper.do_map_mut(self.14), mapper.do_map_mut(self.15), mapper.do_map_mut(self.16), mapper.do_map_mut(self.17), mapper.do_map_mut(self.18), mapper.do_map_mut(self.19), mapper.do_map_mut(self.20), mapper.do_map_mut(self.21), mapper.do_map_mut(self.22), mapper.do_map_mut(self.23), mapper.do_map_mut(self.24), mapper.do_map_mut(self.25), mapper.do_map_mut(self.26), mapper.do_map_mut(self.27), mapper.do_map_mut(self.28), mapper.do_map_mut(self.29), mapper.do_map_mut(self.30), mapper.do_map_mut(self.31))
    }
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30, M31> TupleMapAll<(M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30, M31)> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31)
where
    M0: TupleMapperOnce<Self> + TupleMapperOnceN<0, Self>,
    M1: TupleMapperOnce<Self> + TupleMapperOnceN<1, Self>,
    M2: TupleMapperOnce<Self> + TupleMapperOnceN<2, Self>,
    M3: TupleMapperOnce<Self> + TupleMapperOnceN<3, Self>,
    M4: TupleMapperOnce<Self> + TupleMapperOnceN<4, Self>,
    M5: TupleMapperOnce<Self> + TupleMapperOnceN<5, Self>,
    M6: TupleMapperOnce<Self> + TupleMapperOnceN<6, Self>,
    M7: TupleMapperOnce<Self> + TupleMapperOnceN<7, Self>,
    M8: TupleMapperOnce<Self> + TupleMapperOnceN<8, Self>,
    M9: TupleMapperOnce<Self> + TupleMapperOnceN<9, Self>,
    M10: TupleMapperOnce<Self> + TupleMapperOnceN<10, Self>,
    M11: TupleMapperOnce<Self> + TupleMapperOnceN<11, Self>,
    M12: TupleMapperOnce<Self> + TupleMapperOnceN<12, Self>,
    M13: TupleMapperOnce<Self> + TupleMapperOnceN<13, Self>,
    M14: TupleMapperOnce<Self> + TupleMapperOnceN<14, Self>,
    M15: TupleMapperOnce<Self> + TupleMapperOnceN<15, Self>,
    M16: TupleMapperOnce<Self> + TupleMapperOnceN<16, Self>,
    M17: TupleMapperOnce<Self> + TupleMapperOnceN<17, Self>,
    M18: TupleMapperOnce<Self> + TupleMapperOnceN<18, Self>,
    M19: TupleMapperOnce<Self> + TupleMapperOnceN<19, Self>,
    M20: TupleMapperOnce<Self> + TupleMapperOnceN<20, Self>,
    M21: TupleMapperOnce<Self> + TupleMapperOnceN<21, Self>,
    M22: TupleMapperOnce<Self> + TupleMapperOnceN<22, Self>,
    M23: TupleMapperOnce<Self> + TupleMapperOnceN<23, Self>,
    M24: TupleMapperOnce<Self> + TupleMapperOnceN<24, Self>,
    M25: TupleMapperOnce<Self> + TupleMapperOnceN<25, Self>,
    M26: TupleMapperOnce<Self> + TupleMapperOnceN<26, Self>,
    M27: TupleMapperOnce<Self> + TupleMapperOnceN<27, Self>,
    M28: TupleMapperOnce<Self> + TupleMapperOnceN<28, Self>,
    M29: TupleMapperOnce<Self> + TupleMapperOnceN<29, Self>,
    M30: TupleMapperOnce<Self> + TupleMapperOnceN<30, Self>,
    M31: TupleMapperOnce<Self> + TupleMapperOnceN<31, Self>,
{
    type Output = (M0::Output<0>, M1::Output<1>, M2::Output<2>, M3::Output<3>, M4::Output<4>, M5::Output<5>, M6::Output<6>, M7::Output<7>, M8::Output<8>, M9::Output<9>, M10::Output<10>, M11::Output<11>, M12::Output<12>, M13::Output<13>, M14::Output<14>, M15::Output<15>, M16::Output<16>, M17::Output<17>, M18::Output<18>, M19::Output<19>, M20::Output<20>, M21::Output<21>, M22::Output<22>, M23::Output<23>, M24::Output<24>, M25::Output<25>, M26::Output<26>, M27::Output<27>, M28::Output<28>, M29::Output<29>, M30::Output<30>, M31::Output<31>);
    fn map_all(self, mapper: (M0, M1, M2, M3, M4, M5, M6, M7, M8, M9, M10, M11, M12, M13, M14, M15, M16, M17, M18, M19, M20, M21, M22, M23, M24, M25, M26, M27, M28, M29, M30, M31)) -> Self::Output {
        let (m0, m1, m2, m3, m4, m5, m6, m7, m8, m9, m10, m11, m12, m13, m14, m15, m16, m17, m18, m19, m20, m21, m22, m23, m24, m25, m26, m27, m28, m29, m30, m31) = mapper;
        (m0.do_map_once(self.0), m1.do_map_once(self.1), m2.do_map_once(self.2), m3.do_map_once(self.3), m4.do_map_once(self.4), m5.do_map_once(self.5), m6.do_map_once(self.6), m7.do_map_once(self.7), m8.do_map_once(self.8), m9.do_map_once(self.9), m10.do_map_once(self.10), m11.do_map_once(self.11), m12.do_map_once(self.12), m13.do_map_once(self.13), m14.do_map_once(self.14), m15.do_map_once(self.15), m16.do_map_once(self.16), m17.do_map_once(self.17), m18.do_map_once(self.18), m19.do_map_once(self.19), m20.do_map_once(self.20), m21.do_map_once(self.21), m22.do_map_once(self.22), m23.do_map_once(self.23), m24.do_map_once(self.24), m25.do_map_once(self.25), m26.do_map_once(self.26), m27.do_map_once(self.27), m28.do_map_once(self.28), m29.do_map_once(self.29), m30.do_map_once(self.30), m31.do_map_once(self.31))
    }
}
