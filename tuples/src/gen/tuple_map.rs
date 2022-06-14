// This file is by code gen, do not modify

#[doc = "Mapping `.0` for Tuple2"]
pub trait Tuple2Map0<T0, T1> {
    #[doc = "Mapping `.0` for Tuple2"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1);
}
impl<T0, T1> Tuple2Map0<T0, T1> for (T0, T1) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1) {
        (f(self.0), self.1)
    }
}
#[doc = "Mapping `.0` for Tuple2"]
pub trait Tuple2Map0Option<T0, T1> {
    #[doc = "Mapping `.0` for Tuple2"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1)>;
}
impl<T0, T1> Tuple2Map0Option<T0, T1> for Option<(T0, T1)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple2"]
pub trait Tuple2Map0Result<E, T0, T1> {
    #[doc = "Mapping `.0` for Tuple2"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1), E>;
}
impl<E, T0, T1> Tuple2Map0Result<E, T0, T1> for Result<(T0, T1), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple2"]
pub trait Tuple2Map1<T0, T1> {
    #[doc = "Mapping `.1` for Tuple2"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U);
}
impl<T0, T1> Tuple2Map1<T0, T1> for (T0, T1) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U) {
        (self.0, f(self.1))
    }
}
#[doc = "Mapping `.1` for Tuple2"]
pub trait Tuple2Map1Option<T0, T1> {
    #[doc = "Mapping `.1` for Tuple2"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U)>;
}
impl<T0, T1> Tuple2Map1Option<T0, T1> for Option<(T0, T1)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple2"]
pub trait Tuple2Map1Result<E, T0, T1> {
    #[doc = "Mapping `.1` for Tuple2"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U), E>;
}
impl<E, T0, T1> Tuple2Map1Result<E, T0, T1> for Result<(T0, T1), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple2"]
pub trait Tuple2Map<T> {
    #[doc = "Mapping for Tuple2"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U);
}
impl<T> Tuple2Map<T> for (T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U) {
        (f(self.0), f(self.1))
    }
}
#[doc = "Mapping all item for Tuple2"]
pub trait Tuple2MapAll<T0, T1> {
    #[doc = "Mapping all item for Tuple2"]
    fn map_all<U0, U1>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1) -> (U0, U1);
}
impl<T0, T1> Tuple2MapAll<T0, T1> for (T0, T1) {
    fn map_all<U0, U1>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1) -> (U0, U1) {
        (f0(self.0), f1(self.1))
    }
}
#[doc = "Mapping `.0` for Tuple3"]
pub trait Tuple3Map0<T0, T1, T2> {
    #[doc = "Mapping `.0` for Tuple3"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2);
}
impl<T0, T1, T2> Tuple3Map0<T0, T1, T2> for (T0, T1, T2) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2) {
        (f(self.0), self.1, self.2)
    }
}
#[doc = "Mapping `.0` for Tuple3"]
pub trait Tuple3Map0Option<T0, T1, T2> {
    #[doc = "Mapping `.0` for Tuple3"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2)>;
}
impl<T0, T1, T2> Tuple3Map0Option<T0, T1, T2> for Option<(T0, T1, T2)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple3"]
pub trait Tuple3Map0Result<E, T0, T1, T2> {
    #[doc = "Mapping `.0` for Tuple3"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2), E>;
}
impl<E, T0, T1, T2> Tuple3Map0Result<E, T0, T1, T2> for Result<(T0, T1, T2), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple3"]
pub trait Tuple3Map1<T0, T1, T2> {
    #[doc = "Mapping `.1` for Tuple3"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2);
}
impl<T0, T1, T2> Tuple3Map1<T0, T1, T2> for (T0, T1, T2) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2) {
        (self.0, f(self.1), self.2)
    }
}
#[doc = "Mapping `.1` for Tuple3"]
pub trait Tuple3Map1Option<T0, T1, T2> {
    #[doc = "Mapping `.1` for Tuple3"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2)>;
}
impl<T0, T1, T2> Tuple3Map1Option<T0, T1, T2> for Option<(T0, T1, T2)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple3"]
pub trait Tuple3Map1Result<E, T0, T1, T2> {
    #[doc = "Mapping `.1` for Tuple3"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2), E>;
}
impl<E, T0, T1, T2> Tuple3Map1Result<E, T0, T1, T2> for Result<(T0, T1, T2), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple3"]
pub trait Tuple3Map2<T0, T1, T2> {
    #[doc = "Mapping `.2` for Tuple3"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U);
}
impl<T0, T1, T2> Tuple3Map2<T0, T1, T2> for (T0, T1, T2) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U) {
        (self.0, self.1, f(self.2))
    }
}
#[doc = "Mapping `.2` for Tuple3"]
pub trait Tuple3Map2Option<T0, T1, T2> {
    #[doc = "Mapping `.2` for Tuple3"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U)>;
}
impl<T0, T1, T2> Tuple3Map2Option<T0, T1, T2> for Option<(T0, T1, T2)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple3"]
pub trait Tuple3Map2Result<E, T0, T1, T2> {
    #[doc = "Mapping `.2` for Tuple3"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U), E>;
}
impl<E, T0, T1, T2> Tuple3Map2Result<E, T0, T1, T2> for Result<(T0, T1, T2), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple3"]
pub trait Tuple3Map<T> {
    #[doc = "Mapping for Tuple3"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U);
}
impl<T> Tuple3Map<T> for (T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U) {
        (f(self.0), f(self.1), f(self.2))
    }
}
#[doc = "Mapping all item for Tuple3"]
pub trait Tuple3MapAll<T0, T1, T2> {
    #[doc = "Mapping all item for Tuple3"]
    fn map_all<U0, U1, U2>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2) -> (U0, U1, U2);
}
impl<T0, T1, T2> Tuple3MapAll<T0, T1, T2> for (T0, T1, T2) {
    fn map_all<U0, U1, U2>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2) -> (U0, U1, U2) {
        (f0(self.0), f1(self.1), f2(self.2))
    }
}
#[doc = "Mapping `.0` for Tuple4"]
pub trait Tuple4Map0<T0, T1, T2, T3> {
    #[doc = "Mapping `.0` for Tuple4"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3);
}
impl<T0, T1, T2, T3> Tuple4Map0<T0, T1, T2, T3> for (T0, T1, T2, T3) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3) {
        (f(self.0), self.1, self.2, self.3)
    }
}
#[doc = "Mapping `.0` for Tuple4"]
pub trait Tuple4Map0Option<T0, T1, T2, T3> {
    #[doc = "Mapping `.0` for Tuple4"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3)>;
}
impl<T0, T1, T2, T3> Tuple4Map0Option<T0, T1, T2, T3> for Option<(T0, T1, T2, T3)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple4"]
pub trait Tuple4Map0Result<E, T0, T1, T2, T3> {
    #[doc = "Mapping `.0` for Tuple4"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3), E>;
}
impl<E, T0, T1, T2, T3> Tuple4Map0Result<E, T0, T1, T2, T3> for Result<(T0, T1, T2, T3), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple4"]
pub trait Tuple4Map1<T0, T1, T2, T3> {
    #[doc = "Mapping `.1` for Tuple4"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3);
}
impl<T0, T1, T2, T3> Tuple4Map1<T0, T1, T2, T3> for (T0, T1, T2, T3) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3) {
        (self.0, f(self.1), self.2, self.3)
    }
}
#[doc = "Mapping `.1` for Tuple4"]
pub trait Tuple4Map1Option<T0, T1, T2, T3> {
    #[doc = "Mapping `.1` for Tuple4"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3)>;
}
impl<T0, T1, T2, T3> Tuple4Map1Option<T0, T1, T2, T3> for Option<(T0, T1, T2, T3)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple4"]
pub trait Tuple4Map1Result<E, T0, T1, T2, T3> {
    #[doc = "Mapping `.1` for Tuple4"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3), E>;
}
impl<E, T0, T1, T2, T3> Tuple4Map1Result<E, T0, T1, T2, T3> for Result<(T0, T1, T2, T3), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple4"]
pub trait Tuple4Map2<T0, T1, T2, T3> {
    #[doc = "Mapping `.2` for Tuple4"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3);
}
impl<T0, T1, T2, T3> Tuple4Map2<T0, T1, T2, T3> for (T0, T1, T2, T3) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3) {
        (self.0, self.1, f(self.2), self.3)
    }
}
#[doc = "Mapping `.2` for Tuple4"]
pub trait Tuple4Map2Option<T0, T1, T2, T3> {
    #[doc = "Mapping `.2` for Tuple4"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3)>;
}
impl<T0, T1, T2, T3> Tuple4Map2Option<T0, T1, T2, T3> for Option<(T0, T1, T2, T3)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple4"]
pub trait Tuple4Map2Result<E, T0, T1, T2, T3> {
    #[doc = "Mapping `.2` for Tuple4"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3), E>;
}
impl<E, T0, T1, T2, T3> Tuple4Map2Result<E, T0, T1, T2, T3> for Result<(T0, T1, T2, T3), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple4"]
pub trait Tuple4Map3<T0, T1, T2, T3> {
    #[doc = "Mapping `.3` for Tuple4"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U);
}
impl<T0, T1, T2, T3> Tuple4Map3<T0, T1, T2, T3> for (T0, T1, T2, T3) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U) {
        (self.0, self.1, self.2, f(self.3))
    }
}
#[doc = "Mapping `.3` for Tuple4"]
pub trait Tuple4Map3Option<T0, T1, T2, T3> {
    #[doc = "Mapping `.3` for Tuple4"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U)>;
}
impl<T0, T1, T2, T3> Tuple4Map3Option<T0, T1, T2, T3> for Option<(T0, T1, T2, T3)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple4"]
pub trait Tuple4Map3Result<E, T0, T1, T2, T3> {
    #[doc = "Mapping `.3` for Tuple4"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U), E>;
}
impl<E, T0, T1, T2, T3> Tuple4Map3Result<E, T0, T1, T2, T3> for Result<(T0, T1, T2, T3), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple4"]
pub trait Tuple4Map<T> {
    #[doc = "Mapping for Tuple4"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U);
}
impl<T> Tuple4Map<T> for (T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3))
    }
}
#[doc = "Mapping all item for Tuple4"]
pub trait Tuple4MapAll<T0, T1, T2, T3> {
    #[doc = "Mapping all item for Tuple4"]
    fn map_all<U0, U1, U2, U3>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3) -> (U0, U1, U2, U3);
}
impl<T0, T1, T2, T3> Tuple4MapAll<T0, T1, T2, T3> for (T0, T1, T2, T3) {
    fn map_all<U0, U1, U2, U3>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3) -> (U0, U1, U2, U3) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3))
    }
}
#[doc = "Mapping `.0` for Tuple5"]
pub trait Tuple5Map0<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.0` for Tuple5"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4);
}
impl<T0, T1, T2, T3, T4> Tuple5Map0<T0, T1, T2, T3, T4> for (T0, T1, T2, T3, T4) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4) {
        (f(self.0), self.1, self.2, self.3, self.4)
    }
}
#[doc = "Mapping `.0` for Tuple5"]
pub trait Tuple5Map0Option<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.0` for Tuple5"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4)>;
}
impl<T0, T1, T2, T3, T4> Tuple5Map0Option<T0, T1, T2, T3, T4> for Option<(T0, T1, T2, T3, T4)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple5"]
pub trait Tuple5Map0Result<E, T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.0` for Tuple5"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4), E>;
}
impl<E, T0, T1, T2, T3, T4> Tuple5Map0Result<E, T0, T1, T2, T3, T4> for Result<(T0, T1, T2, T3, T4), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple5"]
pub trait Tuple5Map1<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.1` for Tuple5"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4);
}
impl<T0, T1, T2, T3, T4> Tuple5Map1<T0, T1, T2, T3, T4> for (T0, T1, T2, T3, T4) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4) {
        (self.0, f(self.1), self.2, self.3, self.4)
    }
}
#[doc = "Mapping `.1` for Tuple5"]
pub trait Tuple5Map1Option<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.1` for Tuple5"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4)>;
}
impl<T0, T1, T2, T3, T4> Tuple5Map1Option<T0, T1, T2, T3, T4> for Option<(T0, T1, T2, T3, T4)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple5"]
pub trait Tuple5Map1Result<E, T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.1` for Tuple5"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4), E>;
}
impl<E, T0, T1, T2, T3, T4> Tuple5Map1Result<E, T0, T1, T2, T3, T4> for Result<(T0, T1, T2, T3, T4), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple5"]
pub trait Tuple5Map2<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.2` for Tuple5"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4);
}
impl<T0, T1, T2, T3, T4> Tuple5Map2<T0, T1, T2, T3, T4> for (T0, T1, T2, T3, T4) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4) {
        (self.0, self.1, f(self.2), self.3, self.4)
    }
}
#[doc = "Mapping `.2` for Tuple5"]
pub trait Tuple5Map2Option<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.2` for Tuple5"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4)>;
}
impl<T0, T1, T2, T3, T4> Tuple5Map2Option<T0, T1, T2, T3, T4> for Option<(T0, T1, T2, T3, T4)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple5"]
pub trait Tuple5Map2Result<E, T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.2` for Tuple5"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4), E>;
}
impl<E, T0, T1, T2, T3, T4> Tuple5Map2Result<E, T0, T1, T2, T3, T4> for Result<(T0, T1, T2, T3, T4), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple5"]
pub trait Tuple5Map3<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.3` for Tuple5"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4);
}
impl<T0, T1, T2, T3, T4> Tuple5Map3<T0, T1, T2, T3, T4> for (T0, T1, T2, T3, T4) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4) {
        (self.0, self.1, self.2, f(self.3), self.4)
    }
}
#[doc = "Mapping `.3` for Tuple5"]
pub trait Tuple5Map3Option<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.3` for Tuple5"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4)>;
}
impl<T0, T1, T2, T3, T4> Tuple5Map3Option<T0, T1, T2, T3, T4> for Option<(T0, T1, T2, T3, T4)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple5"]
pub trait Tuple5Map3Result<E, T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.3` for Tuple5"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4), E>;
}
impl<E, T0, T1, T2, T3, T4> Tuple5Map3Result<E, T0, T1, T2, T3, T4> for Result<(T0, T1, T2, T3, T4), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple5"]
pub trait Tuple5Map4<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.4` for Tuple5"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U);
}
impl<T0, T1, T2, T3, T4> Tuple5Map4<T0, T1, T2, T3, T4> for (T0, T1, T2, T3, T4) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U) {
        (self.0, self.1, self.2, self.3, f(self.4))
    }
}
#[doc = "Mapping `.4` for Tuple5"]
pub trait Tuple5Map4Option<T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.4` for Tuple5"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U)>;
}
impl<T0, T1, T2, T3, T4> Tuple5Map4Option<T0, T1, T2, T3, T4> for Option<(T0, T1, T2, T3, T4)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple5"]
pub trait Tuple5Map4Result<E, T0, T1, T2, T3, T4> {
    #[doc = "Mapping `.4` for Tuple5"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U), E>;
}
impl<E, T0, T1, T2, T3, T4> Tuple5Map4Result<E, T0, T1, T2, T3, T4> for Result<(T0, T1, T2, T3, T4), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple5"]
pub trait Tuple5Map<T> {
    #[doc = "Mapping for Tuple5"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U);
}
impl<T> Tuple5Map<T> for (T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4))
    }
}
#[doc = "Mapping all item for Tuple5"]
pub trait Tuple5MapAll<T0, T1, T2, T3, T4> {
    #[doc = "Mapping all item for Tuple5"]
    fn map_all<U0, U1, U2, U3, U4>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4) -> (U0, U1, U2, U3, U4);
}
impl<T0, T1, T2, T3, T4> Tuple5MapAll<T0, T1, T2, T3, T4> for (T0, T1, T2, T3, T4) {
    fn map_all<U0, U1, U2, U3, U4>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4) -> (U0, U1, U2, U3, U4) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4))
    }
}
#[doc = "Mapping `.0` for Tuple6"]
pub trait Tuple6Map0<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.0` for Tuple6"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map0<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5)
    }
}
#[doc = "Mapping `.0` for Tuple6"]
pub trait Tuple6Map0Option<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.0` for Tuple6"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5)>;
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map0Option<T0, T1, T2, T3, T4, T5> for Option<(T0, T1, T2, T3, T4, T5)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple6"]
pub trait Tuple6Map0Result<E, T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.0` for Tuple6"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5), E>;
}
impl<E, T0, T1, T2, T3, T4, T5> Tuple6Map0Result<E, T0, T1, T2, T3, T4, T5> for Result<(T0, T1, T2, T3, T4, T5), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple6"]
pub trait Tuple6Map1<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.1` for Tuple6"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map1<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5)
    }
}
#[doc = "Mapping `.1` for Tuple6"]
pub trait Tuple6Map1Option<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.1` for Tuple6"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5)>;
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map1Option<T0, T1, T2, T3, T4, T5> for Option<(T0, T1, T2, T3, T4, T5)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple6"]
pub trait Tuple6Map1Result<E, T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.1` for Tuple6"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5), E>;
}
impl<E, T0, T1, T2, T3, T4, T5> Tuple6Map1Result<E, T0, T1, T2, T3, T4, T5> for Result<(T0, T1, T2, T3, T4, T5), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple6"]
pub trait Tuple6Map2<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.2` for Tuple6"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map2<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5)
    }
}
#[doc = "Mapping `.2` for Tuple6"]
pub trait Tuple6Map2Option<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.2` for Tuple6"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5)>;
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map2Option<T0, T1, T2, T3, T4, T5> for Option<(T0, T1, T2, T3, T4, T5)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple6"]
pub trait Tuple6Map2Result<E, T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.2` for Tuple6"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5), E>;
}
impl<E, T0, T1, T2, T3, T4, T5> Tuple6Map2Result<E, T0, T1, T2, T3, T4, T5> for Result<(T0, T1, T2, T3, T4, T5), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple6"]
pub trait Tuple6Map3<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.3` for Tuple6"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map3<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5)
    }
}
#[doc = "Mapping `.3` for Tuple6"]
pub trait Tuple6Map3Option<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.3` for Tuple6"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5)>;
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map3Option<T0, T1, T2, T3, T4, T5> for Option<(T0, T1, T2, T3, T4, T5)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple6"]
pub trait Tuple6Map3Result<E, T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.3` for Tuple6"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5), E>;
}
impl<E, T0, T1, T2, T3, T4, T5> Tuple6Map3Result<E, T0, T1, T2, T3, T4, T5> for Result<(T0, T1, T2, T3, T4, T5), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple6"]
pub trait Tuple6Map4<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.4` for Tuple6"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map4<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5)
    }
}
#[doc = "Mapping `.4` for Tuple6"]
pub trait Tuple6Map4Option<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.4` for Tuple6"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5)>;
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map4Option<T0, T1, T2, T3, T4, T5> for Option<(T0, T1, T2, T3, T4, T5)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple6"]
pub trait Tuple6Map4Result<E, T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.4` for Tuple6"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5), E>;
}
impl<E, T0, T1, T2, T3, T4, T5> Tuple6Map4Result<E, T0, T1, T2, T3, T4, T5> for Result<(T0, T1, T2, T3, T4, T5), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple6"]
pub trait Tuple6Map5<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.5` for Tuple6"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map5<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5))
    }
}
#[doc = "Mapping `.5` for Tuple6"]
pub trait Tuple6Map5Option<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.5` for Tuple6"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U)>;
}
impl<T0, T1, T2, T3, T4, T5> Tuple6Map5Option<T0, T1, T2, T3, T4, T5> for Option<(T0, T1, T2, T3, T4, T5)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple6"]
pub trait Tuple6Map5Result<E, T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping `.5` for Tuple6"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5> Tuple6Map5Result<E, T0, T1, T2, T3, T4, T5> for Result<(T0, T1, T2, T3, T4, T5), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple6"]
pub trait Tuple6Map<T> {
    #[doc = "Mapping for Tuple6"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U);
}
impl<T> Tuple6Map<T> for (T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5))
    }
}
#[doc = "Mapping all item for Tuple6"]
pub trait Tuple6MapAll<T0, T1, T2, T3, T4, T5> {
    #[doc = "Mapping all item for Tuple6"]
    fn map_all<U0, U1, U2, U3, U4, U5>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5) -> (U0, U1, U2, U3, U4, U5);
}
impl<T0, T1, T2, T3, T4, T5> Tuple6MapAll<T0, T1, T2, T3, T4, T5> for (T0, T1, T2, T3, T4, T5) {
    fn map_all<U0, U1, U2, U3, U4, U5>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5) -> (U0, U1, U2, U3, U4, U5) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5))
    }
}
#[doc = "Mapping `.0` for Tuple7"]
pub trait Tuple7Map0<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.0` for Tuple7"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map0<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6)
    }
}
#[doc = "Mapping `.0` for Tuple7"]
pub trait Tuple7Map0Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.0` for Tuple7"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map0Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple7"]
pub trait Tuple7Map0Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.0` for Tuple7"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map0Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple7"]
pub trait Tuple7Map1<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.1` for Tuple7"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map1<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6)
    }
}
#[doc = "Mapping `.1` for Tuple7"]
pub trait Tuple7Map1Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.1` for Tuple7"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map1Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple7"]
pub trait Tuple7Map1Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.1` for Tuple7"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map1Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple7"]
pub trait Tuple7Map2<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.2` for Tuple7"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map2<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6)
    }
}
#[doc = "Mapping `.2` for Tuple7"]
pub trait Tuple7Map2Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.2` for Tuple7"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map2Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple7"]
pub trait Tuple7Map2Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.2` for Tuple7"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map2Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple7"]
pub trait Tuple7Map3<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.3` for Tuple7"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map3<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6)
    }
}
#[doc = "Mapping `.3` for Tuple7"]
pub trait Tuple7Map3Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.3` for Tuple7"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map3Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple7"]
pub trait Tuple7Map3Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.3` for Tuple7"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map3Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple7"]
pub trait Tuple7Map4<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.4` for Tuple7"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map4<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6)
    }
}
#[doc = "Mapping `.4` for Tuple7"]
pub trait Tuple7Map4Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.4` for Tuple7"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map4Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple7"]
pub trait Tuple7Map4Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.4` for Tuple7"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map4Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple7"]
pub trait Tuple7Map5<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.5` for Tuple7"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map5<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6)
    }
}
#[doc = "Mapping `.5` for Tuple7"]
pub trait Tuple7Map5Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.5` for Tuple7"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map5Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple7"]
pub trait Tuple7Map5Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.5` for Tuple7"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map5Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple7"]
pub trait Tuple7Map6<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.6` for Tuple7"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map6<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6))
    }
}
#[doc = "Mapping `.6` for Tuple7"]
pub trait Tuple7Map6Option<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.6` for Tuple7"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7Map6Option<T0, T1, T2, T3, T4, T5, T6> for Option<(T0, T1, T2, T3, T4, T5, T6)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple7"]
pub trait Tuple7Map6Result<E, T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping `.6` for Tuple7"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6> Tuple7Map6Result<E, T0, T1, T2, T3, T4, T5, T6> for Result<(T0, T1, T2, T3, T4, T5, T6), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple7"]
pub trait Tuple7Map<T> {
    #[doc = "Mapping for Tuple7"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U);
}
impl<T> Tuple7Map<T> for (T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6))
    }
}
#[doc = "Mapping all item for Tuple7"]
pub trait Tuple7MapAll<T0, T1, T2, T3, T4, T5, T6> {
    #[doc = "Mapping all item for Tuple7"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6) -> (U0, U1, U2, U3, U4, U5, U6);
}
impl<T0, T1, T2, T3, T4, T5, T6> Tuple7MapAll<T0, T1, T2, T3, T4, T5, T6> for (T0, T1, T2, T3, T4, T5, T6) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6) -> (U0, U1, U2, U3, U4, U5, U6) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6))
    }
}
#[doc = "Mapping `.0` for Tuple8"]
pub trait Tuple8Map0<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.0` for Tuple8"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map0<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
#[doc = "Mapping `.0` for Tuple8"]
pub trait Tuple8Map0Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.0` for Tuple8"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map0Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple8"]
pub trait Tuple8Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.0` for Tuple8"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple8"]
pub trait Tuple8Map1<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.1` for Tuple8"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map1<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7)
    }
}
#[doc = "Mapping `.1` for Tuple8"]
pub trait Tuple8Map1Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.1` for Tuple8"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map1Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple8"]
pub trait Tuple8Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.1` for Tuple8"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple8"]
pub trait Tuple8Map2<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.2` for Tuple8"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map2<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7)
    }
}
#[doc = "Mapping `.2` for Tuple8"]
pub trait Tuple8Map2Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.2` for Tuple8"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map2Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple8"]
pub trait Tuple8Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.2` for Tuple8"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple8"]
pub trait Tuple8Map3<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.3` for Tuple8"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map3<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7)
    }
}
#[doc = "Mapping `.3` for Tuple8"]
pub trait Tuple8Map3Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.3` for Tuple8"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map3Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple8"]
pub trait Tuple8Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.3` for Tuple8"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple8"]
pub trait Tuple8Map4<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.4` for Tuple8"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map4<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7)
    }
}
#[doc = "Mapping `.4` for Tuple8"]
pub trait Tuple8Map4Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.4` for Tuple8"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map4Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple8"]
pub trait Tuple8Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.4` for Tuple8"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple8"]
pub trait Tuple8Map5<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.5` for Tuple8"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map5<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7)
    }
}
#[doc = "Mapping `.5` for Tuple8"]
pub trait Tuple8Map5Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.5` for Tuple8"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map5Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple8"]
pub trait Tuple8Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.5` for Tuple8"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple8"]
pub trait Tuple8Map6<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.6` for Tuple8"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map6<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7)
    }
}
#[doc = "Mapping `.6` for Tuple8"]
pub trait Tuple8Map6Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.6` for Tuple8"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map6Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple8"]
pub trait Tuple8Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.6` for Tuple8"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple8"]
pub trait Tuple8Map7<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.7` for Tuple8"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map7<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7))
    }
}
#[doc = "Mapping `.7` for Tuple8"]
pub trait Tuple8Map7Option<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.7` for Tuple8"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map7Option<T0, T1, T2, T3, T4, T5, T6, T7> for Option<(T0, T1, T2, T3, T4, T5, T6, T7)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple8"]
pub trait Tuple8Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping `.7` for Tuple8"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7> Tuple8Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7> for Result<(T0, T1, T2, T3, T4, T5, T6, T7), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple8"]
pub trait Tuple8Map<T> {
    #[doc = "Mapping for Tuple8"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U);
}
impl<T> Tuple8Map<T> for (T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7))
    }
}
#[doc = "Mapping all item for Tuple8"]
pub trait Tuple8MapAll<T0, T1, T2, T3, T4, T5, T6, T7> {
    #[doc = "Mapping all item for Tuple8"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7) -> (U0, U1, U2, U3, U4, U5, U6, U7);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7> Tuple8MapAll<T0, T1, T2, T3, T4, T5, T6, T7> for (T0, T1, T2, T3, T4, T5, T6, T7) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7) -> (U0, U1, U2, U3, U4, U5, U6, U7) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7))
    }
}
#[doc = "Mapping `.0` for Tuple9"]
pub trait Tuple9Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.0` for Tuple9"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
#[doc = "Mapping `.0` for Tuple9"]
pub trait Tuple9Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.0` for Tuple9"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple9"]
pub trait Tuple9Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.0` for Tuple9"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple9"]
pub trait Tuple9Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.1` for Tuple9"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
#[doc = "Mapping `.1` for Tuple9"]
pub trait Tuple9Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.1` for Tuple9"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple9"]
pub trait Tuple9Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.1` for Tuple9"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple9"]
pub trait Tuple9Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.2` for Tuple9"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8)
    }
}
#[doc = "Mapping `.2` for Tuple9"]
pub trait Tuple9Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.2` for Tuple9"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple9"]
pub trait Tuple9Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.2` for Tuple9"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple9"]
pub trait Tuple9Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.3` for Tuple9"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8)
    }
}
#[doc = "Mapping `.3` for Tuple9"]
pub trait Tuple9Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.3` for Tuple9"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple9"]
pub trait Tuple9Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.3` for Tuple9"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple9"]
pub trait Tuple9Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.4` for Tuple9"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8)
    }
}
#[doc = "Mapping `.4` for Tuple9"]
pub trait Tuple9Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.4` for Tuple9"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple9"]
pub trait Tuple9Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.4` for Tuple9"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple9"]
pub trait Tuple9Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.5` for Tuple9"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8)
    }
}
#[doc = "Mapping `.5` for Tuple9"]
pub trait Tuple9Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.5` for Tuple9"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple9"]
pub trait Tuple9Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.5` for Tuple9"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple9"]
pub trait Tuple9Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.6` for Tuple9"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8)
    }
}
#[doc = "Mapping `.6` for Tuple9"]
pub trait Tuple9Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.6` for Tuple9"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple9"]
pub trait Tuple9Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.6` for Tuple9"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple9"]
pub trait Tuple9Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.7` for Tuple9"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8)
    }
}
#[doc = "Mapping `.7` for Tuple9"]
pub trait Tuple9Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.7` for Tuple9"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple9"]
pub trait Tuple9Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.7` for Tuple9"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple9"]
pub trait Tuple9Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.8` for Tuple9"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8))
    }
}
#[doc = "Mapping `.8` for Tuple9"]
pub trait Tuple9Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.8` for Tuple9"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple9"]
pub trait Tuple9Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping `.8` for Tuple9"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple9"]
pub trait Tuple9Map<T> {
    #[doc = "Mapping for Tuple9"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple9Map<T> for (T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8))
    }
}
#[doc = "Mapping all item for Tuple9"]
pub trait Tuple9MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8> {
    #[doc = "Mapping all item for Tuple9"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8> Tuple9MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8> for (T0, T1, T2, T3, T4, T5, T6, T7, T8) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8))
    }
}
#[doc = "Mapping `.0` for Tuple10"]
pub trait Tuple10Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.0` for Tuple10"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.0` for Tuple10"]
pub trait Tuple10Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.0` for Tuple10"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple10"]
pub trait Tuple10Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.0` for Tuple10"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple10"]
pub trait Tuple10Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.1` for Tuple10"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.1` for Tuple10"]
pub trait Tuple10Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.1` for Tuple10"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple10"]
pub trait Tuple10Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.1` for Tuple10"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple10"]
pub trait Tuple10Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.2` for Tuple10"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.2` for Tuple10"]
pub trait Tuple10Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.2` for Tuple10"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple10"]
pub trait Tuple10Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.2` for Tuple10"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple10"]
pub trait Tuple10Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.3` for Tuple10"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.3` for Tuple10"]
pub trait Tuple10Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.3` for Tuple10"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple10"]
pub trait Tuple10Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.3` for Tuple10"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple10"]
pub trait Tuple10Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.4` for Tuple10"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.4` for Tuple10"]
pub trait Tuple10Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.4` for Tuple10"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple10"]
pub trait Tuple10Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.4` for Tuple10"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple10"]
pub trait Tuple10Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.5` for Tuple10"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.5` for Tuple10"]
pub trait Tuple10Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.5` for Tuple10"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple10"]
pub trait Tuple10Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.5` for Tuple10"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple10"]
pub trait Tuple10Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.6` for Tuple10"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9)
    }
}
#[doc = "Mapping `.6` for Tuple10"]
pub trait Tuple10Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.6` for Tuple10"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple10"]
pub trait Tuple10Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.6` for Tuple10"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple10"]
pub trait Tuple10Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.7` for Tuple10"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9)
    }
}
#[doc = "Mapping `.7` for Tuple10"]
pub trait Tuple10Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.7` for Tuple10"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple10"]
pub trait Tuple10Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.7` for Tuple10"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple10"]
pub trait Tuple10Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.8` for Tuple10"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9)
    }
}
#[doc = "Mapping `.8` for Tuple10"]
pub trait Tuple10Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.8` for Tuple10"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple10"]
pub trait Tuple10Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.8` for Tuple10"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple10"]
pub trait Tuple10Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.9` for Tuple10"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9))
    }
}
#[doc = "Mapping `.9` for Tuple10"]
pub trait Tuple10Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.9` for Tuple10"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple10"]
pub trait Tuple10Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping `.9` for Tuple10"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple10"]
pub trait Tuple10Map<T> {
    #[doc = "Mapping for Tuple10"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple10Map<T> for (T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9))
    }
}
#[doc = "Mapping all item for Tuple10"]
pub trait Tuple10MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> {
    #[doc = "Mapping all item for Tuple10"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> Tuple10MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9))
    }
}
#[doc = "Mapping `.0` for Tuple11"]
pub trait Tuple11Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.0` for Tuple11"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.0` for Tuple11"]
pub trait Tuple11Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.0` for Tuple11"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple11"]
pub trait Tuple11Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.0` for Tuple11"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple11"]
pub trait Tuple11Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.1` for Tuple11"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.1` for Tuple11"]
pub trait Tuple11Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.1` for Tuple11"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple11"]
pub trait Tuple11Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.1` for Tuple11"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple11"]
pub trait Tuple11Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.2` for Tuple11"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.2` for Tuple11"]
pub trait Tuple11Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.2` for Tuple11"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple11"]
pub trait Tuple11Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.2` for Tuple11"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple11"]
pub trait Tuple11Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.3` for Tuple11"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.3` for Tuple11"]
pub trait Tuple11Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.3` for Tuple11"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple11"]
pub trait Tuple11Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.3` for Tuple11"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple11"]
pub trait Tuple11Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.4` for Tuple11"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.4` for Tuple11"]
pub trait Tuple11Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.4` for Tuple11"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple11"]
pub trait Tuple11Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.4` for Tuple11"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple11"]
pub trait Tuple11Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.5` for Tuple11"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.5` for Tuple11"]
pub trait Tuple11Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.5` for Tuple11"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple11"]
pub trait Tuple11Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.5` for Tuple11"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple11"]
pub trait Tuple11Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.6` for Tuple11"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.6` for Tuple11"]
pub trait Tuple11Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.6` for Tuple11"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple11"]
pub trait Tuple11Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.6` for Tuple11"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple11"]
pub trait Tuple11Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.7` for Tuple11"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9, self.10)
    }
}
#[doc = "Mapping `.7` for Tuple11"]
pub trait Tuple11Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.7` for Tuple11"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple11"]
pub trait Tuple11Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.7` for Tuple11"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple11"]
pub trait Tuple11Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.8` for Tuple11"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9, self.10)
    }
}
#[doc = "Mapping `.8` for Tuple11"]
pub trait Tuple11Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.8` for Tuple11"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple11"]
pub trait Tuple11Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.8` for Tuple11"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple11"]
pub trait Tuple11Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.9` for Tuple11"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9), self.10)
    }
}
#[doc = "Mapping `.9` for Tuple11"]
pub trait Tuple11Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.9` for Tuple11"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple11"]
pub trait Tuple11Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.9` for Tuple11"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.10` for Tuple11"]
pub trait Tuple11Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.10` for Tuple11"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, f(self.10))
    }
}
#[doc = "Mapping `.10` for Tuple11"]
pub trait Tuple11Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.10` for Tuple11"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U)> {
        match self {
            Some(v) => Some(v.map10(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.10` for Tuple11"]
pub trait Tuple11Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping `.10` for Tuple11"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10), E> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U), E> {
        match self {
            Ok(v) => Ok(v.map10(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple11"]
pub trait Tuple11Map<T> {
    #[doc = "Mapping for Tuple11"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple11Map<T> for (T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10))
    }
}
#[doc = "Mapping all item for Tuple11"]
pub trait Tuple11MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
    #[doc = "Mapping all item for Tuple11"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> Tuple11MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10))
    }
}
#[doc = "Mapping `.0` for Tuple12"]
pub trait Tuple12Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.0` for Tuple12"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.0` for Tuple12"]
pub trait Tuple12Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.0` for Tuple12"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple12"]
pub trait Tuple12Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.0` for Tuple12"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple12"]
pub trait Tuple12Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.1` for Tuple12"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.1` for Tuple12"]
pub trait Tuple12Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.1` for Tuple12"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple12"]
pub trait Tuple12Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.1` for Tuple12"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple12"]
pub trait Tuple12Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.2` for Tuple12"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.2` for Tuple12"]
pub trait Tuple12Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.2` for Tuple12"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple12"]
pub trait Tuple12Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.2` for Tuple12"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple12"]
pub trait Tuple12Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.3` for Tuple12"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.3` for Tuple12"]
pub trait Tuple12Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.3` for Tuple12"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple12"]
pub trait Tuple12Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.3` for Tuple12"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple12"]
pub trait Tuple12Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.4` for Tuple12"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.4` for Tuple12"]
pub trait Tuple12Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.4` for Tuple12"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple12"]
pub trait Tuple12Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.4` for Tuple12"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple12"]
pub trait Tuple12Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.5` for Tuple12"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.5` for Tuple12"]
pub trait Tuple12Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.5` for Tuple12"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple12"]
pub trait Tuple12Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.5` for Tuple12"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple12"]
pub trait Tuple12Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.6` for Tuple12"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.6` for Tuple12"]
pub trait Tuple12Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.6` for Tuple12"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple12"]
pub trait Tuple12Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.6` for Tuple12"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple12"]
pub trait Tuple12Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.7` for Tuple12"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.7` for Tuple12"]
pub trait Tuple12Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.7` for Tuple12"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple12"]
pub trait Tuple12Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.7` for Tuple12"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple12"]
pub trait Tuple12Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.8` for Tuple12"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9, self.10, self.11)
    }
}
#[doc = "Mapping `.8` for Tuple12"]
pub trait Tuple12Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.8` for Tuple12"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple12"]
pub trait Tuple12Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.8` for Tuple12"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple12"]
pub trait Tuple12Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.9` for Tuple12"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9), self.10, self.11)
    }
}
#[doc = "Mapping `.9` for Tuple12"]
pub trait Tuple12Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.9` for Tuple12"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple12"]
pub trait Tuple12Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.9` for Tuple12"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.10` for Tuple12"]
pub trait Tuple12Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.10` for Tuple12"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, f(self.10), self.11)
    }
}
#[doc = "Mapping `.10` for Tuple12"]
pub trait Tuple12Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.10` for Tuple12"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11)> {
        match self {
            Some(v) => Some(v.map10(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.10` for Tuple12"]
pub trait Tuple12Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.10` for Tuple12"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11), E> {
        match self {
            Ok(v) => Ok(v.map10(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.11` for Tuple12"]
pub trait Tuple12Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.11` for Tuple12"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, f(self.11))
    }
}
#[doc = "Mapping `.11` for Tuple12"]
pub trait Tuple12Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.11` for Tuple12"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U)> {
        match self {
            Some(v) => Some(v.map11(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.11` for Tuple12"]
pub trait Tuple12Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping `.11` for Tuple12"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11), E> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U), E> {
        match self {
            Ok(v) => Ok(v.map11(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple12"]
pub trait Tuple12Map<T> {
    #[doc = "Mapping for Tuple12"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple12Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11))
    }
}
#[doc = "Mapping all item for Tuple12"]
pub trait Tuple12MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
    #[doc = "Mapping all item for Tuple12"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> Tuple12MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11))
    }
}
#[doc = "Mapping `.0` for Tuple13"]
pub trait Tuple13Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.0` for Tuple13"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.0` for Tuple13"]
pub trait Tuple13Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.0` for Tuple13"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple13"]
pub trait Tuple13Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.0` for Tuple13"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple13"]
pub trait Tuple13Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.1` for Tuple13"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.1` for Tuple13"]
pub trait Tuple13Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.1` for Tuple13"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple13"]
pub trait Tuple13Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.1` for Tuple13"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple13"]
pub trait Tuple13Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.2` for Tuple13"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.2` for Tuple13"]
pub trait Tuple13Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.2` for Tuple13"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple13"]
pub trait Tuple13Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.2` for Tuple13"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple13"]
pub trait Tuple13Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.3` for Tuple13"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.3` for Tuple13"]
pub trait Tuple13Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.3` for Tuple13"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple13"]
pub trait Tuple13Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.3` for Tuple13"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple13"]
pub trait Tuple13Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.4` for Tuple13"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.4` for Tuple13"]
pub trait Tuple13Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.4` for Tuple13"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple13"]
pub trait Tuple13Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.4` for Tuple13"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple13"]
pub trait Tuple13Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.5` for Tuple13"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.5` for Tuple13"]
pub trait Tuple13Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.5` for Tuple13"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple13"]
pub trait Tuple13Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.5` for Tuple13"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple13"]
pub trait Tuple13Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.6` for Tuple13"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.6` for Tuple13"]
pub trait Tuple13Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.6` for Tuple13"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple13"]
pub trait Tuple13Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.6` for Tuple13"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple13"]
pub trait Tuple13Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.7` for Tuple13"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.7` for Tuple13"]
pub trait Tuple13Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.7` for Tuple13"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple13"]
pub trait Tuple13Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.7` for Tuple13"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple13"]
pub trait Tuple13Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.8` for Tuple13"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9, self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.8` for Tuple13"]
pub trait Tuple13Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.8` for Tuple13"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple13"]
pub trait Tuple13Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.8` for Tuple13"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple13"]
pub trait Tuple13Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.9` for Tuple13"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9), self.10, self.11, self.12)
    }
}
#[doc = "Mapping `.9` for Tuple13"]
pub trait Tuple13Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.9` for Tuple13"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple13"]
pub trait Tuple13Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.9` for Tuple13"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.10` for Tuple13"]
pub trait Tuple13Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.10` for Tuple13"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, f(self.10), self.11, self.12)
    }
}
#[doc = "Mapping `.10` for Tuple13"]
pub trait Tuple13Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.10` for Tuple13"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12)> {
        match self {
            Some(v) => Some(v.map10(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.10` for Tuple13"]
pub trait Tuple13Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.10` for Tuple13"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12), E> {
        match self {
            Ok(v) => Ok(v.map10(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.11` for Tuple13"]
pub trait Tuple13Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.11` for Tuple13"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, f(self.11), self.12)
    }
}
#[doc = "Mapping `.11` for Tuple13"]
pub trait Tuple13Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.11` for Tuple13"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12)> {
        match self {
            Some(v) => Some(v.map11(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.11` for Tuple13"]
pub trait Tuple13Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.11` for Tuple13"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12), E> {
        match self {
            Ok(v) => Ok(v.map11(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.12` for Tuple13"]
pub trait Tuple13Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.12` for Tuple13"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, f(self.12))
    }
}
#[doc = "Mapping `.12` for Tuple13"]
pub trait Tuple13Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.12` for Tuple13"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U)> {
        match self {
            Some(v) => Some(v.map12(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.12` for Tuple13"]
pub trait Tuple13Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping `.12` for Tuple13"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12), E> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U), E> {
        match self {
            Ok(v) => Ok(v.map12(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple13"]
pub trait Tuple13Map<T> {
    #[doc = "Mapping for Tuple13"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple13Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12))
    }
}
#[doc = "Mapping all item for Tuple13"]
pub trait Tuple13MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
    #[doc = "Mapping all item for Tuple13"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> Tuple13MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12))
    }
}
#[doc = "Mapping `.0` for Tuple14"]
pub trait Tuple14Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.0` for Tuple14"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.0` for Tuple14"]
pub trait Tuple14Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.0` for Tuple14"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple14"]
pub trait Tuple14Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.0` for Tuple14"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple14"]
pub trait Tuple14Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.1` for Tuple14"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.1` for Tuple14"]
pub trait Tuple14Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.1` for Tuple14"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple14"]
pub trait Tuple14Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.1` for Tuple14"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple14"]
pub trait Tuple14Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.2` for Tuple14"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.2` for Tuple14"]
pub trait Tuple14Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.2` for Tuple14"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple14"]
pub trait Tuple14Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.2` for Tuple14"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple14"]
pub trait Tuple14Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.3` for Tuple14"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.3` for Tuple14"]
pub trait Tuple14Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.3` for Tuple14"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple14"]
pub trait Tuple14Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.3` for Tuple14"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple14"]
pub trait Tuple14Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.4` for Tuple14"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.4` for Tuple14"]
pub trait Tuple14Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.4` for Tuple14"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple14"]
pub trait Tuple14Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.4` for Tuple14"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple14"]
pub trait Tuple14Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.5` for Tuple14"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.5` for Tuple14"]
pub trait Tuple14Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.5` for Tuple14"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple14"]
pub trait Tuple14Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.5` for Tuple14"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple14"]
pub trait Tuple14Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.6` for Tuple14"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.6` for Tuple14"]
pub trait Tuple14Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.6` for Tuple14"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple14"]
pub trait Tuple14Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.6` for Tuple14"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple14"]
pub trait Tuple14Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.7` for Tuple14"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.7` for Tuple14"]
pub trait Tuple14Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.7` for Tuple14"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple14"]
pub trait Tuple14Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.7` for Tuple14"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple14"]
pub trait Tuple14Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.8` for Tuple14"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9, self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.8` for Tuple14"]
pub trait Tuple14Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.8` for Tuple14"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple14"]
pub trait Tuple14Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.8` for Tuple14"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple14"]
pub trait Tuple14Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.9` for Tuple14"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9), self.10, self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.9` for Tuple14"]
pub trait Tuple14Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.9` for Tuple14"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple14"]
pub trait Tuple14Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.9` for Tuple14"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.10` for Tuple14"]
pub trait Tuple14Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.10` for Tuple14"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, f(self.10), self.11, self.12, self.13)
    }
}
#[doc = "Mapping `.10` for Tuple14"]
pub trait Tuple14Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.10` for Tuple14"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13)> {
        match self {
            Some(v) => Some(v.map10(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.10` for Tuple14"]
pub trait Tuple14Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.10` for Tuple14"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map10(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.11` for Tuple14"]
pub trait Tuple14Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.11` for Tuple14"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, f(self.11), self.12, self.13)
    }
}
#[doc = "Mapping `.11` for Tuple14"]
pub trait Tuple14Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.11` for Tuple14"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13)> {
        match self {
            Some(v) => Some(v.map11(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.11` for Tuple14"]
pub trait Tuple14Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.11` for Tuple14"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13), E> {
        match self {
            Ok(v) => Ok(v.map11(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.12` for Tuple14"]
pub trait Tuple14Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.12` for Tuple14"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, f(self.12), self.13)
    }
}
#[doc = "Mapping `.12` for Tuple14"]
pub trait Tuple14Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.12` for Tuple14"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13)> {
        match self {
            Some(v) => Some(v.map12(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.12` for Tuple14"]
pub trait Tuple14Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.12` for Tuple14"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13), E> {
        match self {
            Ok(v) => Ok(v.map12(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.13` for Tuple14"]
pub trait Tuple14Map13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.13` for Tuple14"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, f(self.13))
    }
}
#[doc = "Mapping `.13` for Tuple14"]
pub trait Tuple14Map13Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.13` for Tuple14"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map13Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)> {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U)> {
        match self {
            Some(v) => Some(v.map13(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.13` for Tuple14"]
pub trait Tuple14Map13Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping `.13` for Tuple14"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14Map13Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13), E> {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U), E> {
        match self {
            Ok(v) => Ok(v.map13(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple14"]
pub trait Tuple14Map<T> {
    #[doc = "Mapping for Tuple14"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple14Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13))
    }
}
#[doc = "Mapping all item for Tuple14"]
pub trait Tuple14MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
    #[doc = "Mapping all item for Tuple14"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> Tuple14MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13))
    }
}
#[doc = "Mapping `.0` for Tuple15"]
pub trait Tuple15Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.0` for Tuple15"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.0` for Tuple15"]
pub trait Tuple15Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.0` for Tuple15"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple15"]
pub trait Tuple15Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.0` for Tuple15"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple15"]
pub trait Tuple15Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.1` for Tuple15"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.1` for Tuple15"]
pub trait Tuple15Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.1` for Tuple15"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple15"]
pub trait Tuple15Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.1` for Tuple15"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple15"]
pub trait Tuple15Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.2` for Tuple15"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.2` for Tuple15"]
pub trait Tuple15Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.2` for Tuple15"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple15"]
pub trait Tuple15Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.2` for Tuple15"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple15"]
pub trait Tuple15Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.3` for Tuple15"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.3` for Tuple15"]
pub trait Tuple15Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.3` for Tuple15"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple15"]
pub trait Tuple15Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.3` for Tuple15"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple15"]
pub trait Tuple15Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.4` for Tuple15"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.4` for Tuple15"]
pub trait Tuple15Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.4` for Tuple15"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple15"]
pub trait Tuple15Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.4` for Tuple15"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple15"]
pub trait Tuple15Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.5` for Tuple15"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.5` for Tuple15"]
pub trait Tuple15Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.5` for Tuple15"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple15"]
pub trait Tuple15Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.5` for Tuple15"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple15"]
pub trait Tuple15Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.6` for Tuple15"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.6` for Tuple15"]
pub trait Tuple15Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.6` for Tuple15"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple15"]
pub trait Tuple15Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.6` for Tuple15"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple15"]
pub trait Tuple15Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.7` for Tuple15"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.7` for Tuple15"]
pub trait Tuple15Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.7` for Tuple15"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple15"]
pub trait Tuple15Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.7` for Tuple15"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple15"]
pub trait Tuple15Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.8` for Tuple15"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9, self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.8` for Tuple15"]
pub trait Tuple15Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.8` for Tuple15"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple15"]
pub trait Tuple15Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.8` for Tuple15"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple15"]
pub trait Tuple15Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.9` for Tuple15"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9), self.10, self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.9` for Tuple15"]
pub trait Tuple15Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.9` for Tuple15"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple15"]
pub trait Tuple15Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.9` for Tuple15"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.10` for Tuple15"]
pub trait Tuple15Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.10` for Tuple15"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, f(self.10), self.11, self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.10` for Tuple15"]
pub trait Tuple15Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.10` for Tuple15"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map10(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.10` for Tuple15"]
pub trait Tuple15Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.10` for Tuple15"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map10(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.11` for Tuple15"]
pub trait Tuple15Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.11` for Tuple15"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, f(self.11), self.12, self.13, self.14)
    }
}
#[doc = "Mapping `.11` for Tuple15"]
pub trait Tuple15Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.11` for Tuple15"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14)> {
        match self {
            Some(v) => Some(v.map11(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.11` for Tuple15"]
pub trait Tuple15Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.11` for Tuple15"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map11(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.12` for Tuple15"]
pub trait Tuple15Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.12` for Tuple15"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, f(self.12), self.13, self.14)
    }
}
#[doc = "Mapping `.12` for Tuple15"]
pub trait Tuple15Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.12` for Tuple15"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14)> {
        match self {
            Some(v) => Some(v.map12(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.12` for Tuple15"]
pub trait Tuple15Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.12` for Tuple15"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14), E> {
        match self {
            Ok(v) => Ok(v.map12(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.13` for Tuple15"]
pub trait Tuple15Map13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.13` for Tuple15"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, f(self.13), self.14)
    }
}
#[doc = "Mapping `.13` for Tuple15"]
pub trait Tuple15Map13Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.13` for Tuple15"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map13Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14)> {
        match self {
            Some(v) => Some(v.map13(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.13` for Tuple15"]
pub trait Tuple15Map13Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.13` for Tuple15"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map13Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14), E> {
        match self {
            Ok(v) => Ok(v.map13(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.14` for Tuple15"]
pub trait Tuple15Map14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.14` for Tuple15"]
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, f(self.14))
    }
}
#[doc = "Mapping `.14` for Tuple15"]
pub trait Tuple15Map14Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.14` for Tuple15"]
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map14Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14)> {
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U)> {
        match self {
            Some(v) => Some(v.map14(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.14` for Tuple15"]
pub trait Tuple15Map14Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping `.14` for Tuple15"]
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15Map14Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14), E> {
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U), E> {
        match self {
            Ok(v) => Ok(v.map14(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple15"]
pub trait Tuple15Map<T> {
    #[doc = "Mapping for Tuple15"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple15Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14))
    }
}
#[doc = "Mapping all item for Tuple15"]
pub trait Tuple15MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
    #[doc = "Mapping all item for Tuple15"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> Tuple15MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14))
    }
}
#[doc = "Mapping `.0` for Tuple16"]
pub trait Tuple16Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.0` for Tuple16"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map0<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> (U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (f(self.0), self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.0` for Tuple16"]
pub trait Tuple16Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.0` for Tuple16"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map0Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Option<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map0(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.0` for Tuple16"]
pub trait Tuple16Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.0` for Tuple16"]
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map0Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map0<U>(self, f: impl FnOnce(T0) -> U) -> Result<(U, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map0(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.1` for Tuple16"]
pub trait Tuple16Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.1` for Tuple16"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map1<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> (T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, f(self.1), self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.1` for Tuple16"]
pub trait Tuple16Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.1` for Tuple16"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map1Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Option<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map1(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.1` for Tuple16"]
pub trait Tuple16Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.1` for Tuple16"]
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map1Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map1<U>(self, f: impl FnOnce(T1) -> U) -> Result<(T0, U, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map1(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.2` for Tuple16"]
pub trait Tuple16Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.2` for Tuple16"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map2<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> (T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, f(self.2), self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.2` for Tuple16"]
pub trait Tuple16Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.2` for Tuple16"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map2Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Option<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map2(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.2` for Tuple16"]
pub trait Tuple16Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.2` for Tuple16"]
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map2Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map2<U>(self, f: impl FnOnce(T2) -> U) -> Result<(T0, T1, U, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map2(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.3` for Tuple16"]
pub trait Tuple16Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.3` for Tuple16"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map3<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> (T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, f(self.3), self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.3` for Tuple16"]
pub trait Tuple16Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.3` for Tuple16"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map3Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Option<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map3(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.3` for Tuple16"]
pub trait Tuple16Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.3` for Tuple16"]
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map3Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map3<U>(self, f: impl FnOnce(T3) -> U) -> Result<(T0, T1, T2, U, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map3(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.4` for Tuple16"]
pub trait Tuple16Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.4` for Tuple16"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map4<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> (T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, f(self.4), self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.4` for Tuple16"]
pub trait Tuple16Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.4` for Tuple16"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map4Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Option<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map4(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.4` for Tuple16"]
pub trait Tuple16Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.4` for Tuple16"]
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map4Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map4<U>(self, f: impl FnOnce(T4) -> U) -> Result<(T0, T1, T2, T3, U, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map4(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.5` for Tuple16"]
pub trait Tuple16Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.5` for Tuple16"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map5<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> (T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, f(self.5), self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.5` for Tuple16"]
pub trait Tuple16Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.5` for Tuple16"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map5Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Option<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map5(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.5` for Tuple16"]
pub trait Tuple16Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.5` for Tuple16"]
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map5Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map5<U>(self, f: impl FnOnce(T5) -> U) -> Result<(T0, T1, T2, T3, T4, U, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map5(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.6` for Tuple16"]
pub trait Tuple16Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.6` for Tuple16"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map6<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> (T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, f(self.6), self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.6` for Tuple16"]
pub trait Tuple16Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.6` for Tuple16"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map6Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Option<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map6(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.6` for Tuple16"]
pub trait Tuple16Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.6` for Tuple16"]
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map6Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map6<U>(self, f: impl FnOnce(T6) -> U) -> Result<(T0, T1, T2, T3, T4, T5, U, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map6(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.7` for Tuple16"]
pub trait Tuple16Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.7` for Tuple16"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map7<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> (T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, f(self.7), self.8, self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.7` for Tuple16"]
pub trait Tuple16Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.7` for Tuple16"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map7Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map7(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.7` for Tuple16"]
pub trait Tuple16Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.7` for Tuple16"]
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map7Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map7<U>(self, f: impl FnOnce(T7) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, U, T8, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map7(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.8` for Tuple16"]
pub trait Tuple16Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.8` for Tuple16"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map8<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, f(self.8), self.9, self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.8` for Tuple16"]
pub trait Tuple16Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.8` for Tuple16"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map8Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map8(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.8` for Tuple16"]
pub trait Tuple16Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.8` for Tuple16"]
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map8Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map8<U>(self, f: impl FnOnce(T8) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, U, T9, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map8(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.9` for Tuple16"]
pub trait Tuple16Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.9` for Tuple16"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map9<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, f(self.9), self.10, self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.9` for Tuple16"]
pub trait Tuple16Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.9` for Tuple16"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map9Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map9(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.9` for Tuple16"]
pub trait Tuple16Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.9` for Tuple16"]
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map9Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map9<U>(self, f: impl FnOnce(T9) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, U, T10, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map9(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.10` for Tuple16"]
pub trait Tuple16Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.10` for Tuple16"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, f(self.10), self.11, self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.10` for Tuple16"]
pub trait Tuple16Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.10` for Tuple16"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map10Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map10(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.10` for Tuple16"]
pub trait Tuple16Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.10` for Tuple16"]
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map10Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map10<U>(self, f: impl FnOnce(T10) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, U, T11, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map10(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.11` for Tuple16"]
pub trait Tuple16Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.11` for Tuple16"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, f(self.11), self.12, self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.11` for Tuple16"]
pub trait Tuple16Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.11` for Tuple16"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map11Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map11(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.11` for Tuple16"]
pub trait Tuple16Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.11` for Tuple16"]
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map11Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map11<U>(self, f: impl FnOnce(T11) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, U, T12, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map11(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.12` for Tuple16"]
pub trait Tuple16Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.12` for Tuple16"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, f(self.12), self.13, self.14, self.15)
    }
}
#[doc = "Mapping `.12` for Tuple16"]
pub trait Tuple16Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.12` for Tuple16"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map12Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14, T15)> {
        match self {
            Some(v) => Some(v.map12(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.12` for Tuple16"]
pub trait Tuple16Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.12` for Tuple16"]
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map12Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map12<U>(self, f: impl FnOnce(T12) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, U, T13, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map12(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.13` for Tuple16"]
pub trait Tuple16Map13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.13` for Tuple16"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, f(self.13), self.14, self.15)
    }
}
#[doc = "Mapping `.13` for Tuple16"]
pub trait Tuple16Map13Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.13` for Tuple16"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map13Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14, T15)> {
        match self {
            Some(v) => Some(v.map13(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.13` for Tuple16"]
pub trait Tuple16Map13Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.13` for Tuple16"]
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map13Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map13<U>(self, f: impl FnOnce(T13) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, U, T14, T15), E> {
        match self {
            Ok(v) => Ok(v.map13(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.14` for Tuple16"]
pub trait Tuple16Map14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.14` for Tuple16"]
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U, T15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U, T15) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, f(self.14), self.15)
    }
}
#[doc = "Mapping `.14` for Tuple16"]
pub trait Tuple16Map14Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.14` for Tuple16"]
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U, T15)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map14Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U, T15)> {
        match self {
            Some(v) => Some(v.map14(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.14` for Tuple16"]
pub trait Tuple16Map14Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.14` for Tuple16"]
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U, T15), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map14Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map14<U>(self, f: impl FnOnce(T14) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, U, T15), E> {
        match self {
            Ok(v) => Ok(v.map14(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping `.15` for Tuple16"]
pub trait Tuple16Map15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.15` for Tuple16"]
    fn map15<U>(self, f: impl FnOnce(T15) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map15<U>(self, f: impl FnOnce(T15) -> U) -> (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U) {
        (self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8, self.9, self.10, self.11, self.12, self.13, self.14, f(self.15))
    }
}
#[doc = "Mapping `.15` for Tuple16"]
pub trait Tuple16Map15Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.15` for Tuple16"]
    fn map15<U>(self, f: impl FnOnce(T15) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U)>;
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map15Option<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15)> {
    fn map15<U>(self, f: impl FnOnce(T15) -> U) -> Option<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U)> {
        match self {
            Some(v) => Some(v.map15(f)),
            None => None,
        }
    }
}
#[doc = "Mapping `.15` for Tuple16"]
pub trait Tuple16Map15Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping `.15` for Tuple16"]
    fn map15<U>(self, f: impl FnOnce(T15) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U), E>;
}
impl<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16Map15Result<E, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15), E> {
    fn map15<U>(self, f: impl FnOnce(T15) -> U) -> Result<(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, U), E> {
        match self {
            Ok(v) => Ok(v.map15(f)),
            Err(e) => Err(e),
        }
    }
}
#[doc = "Mapping for Tuple16"]
pub trait Tuple16Map<T> {
    #[doc = "Mapping for Tuple16"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple16Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15))
    }
}
#[doc = "Mapping all item for Tuple16"]
pub trait Tuple16MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
    #[doc = "Mapping all item for Tuple16"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> Tuple16MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15))
    }
}
#[doc = "Mapping for Tuple17"]
pub trait Tuple17Map<T> {
    #[doc = "Mapping for Tuple17"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple17Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16))
    }
}
#[doc = "Mapping all item for Tuple17"]
pub trait Tuple17MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> {
    #[doc = "Mapping all item for Tuple17"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> Tuple17MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16))
    }
}
#[doc = "Mapping for Tuple18"]
pub trait Tuple18Map<T> {
    #[doc = "Mapping for Tuple18"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple18Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17))
    }
}
#[doc = "Mapping all item for Tuple18"]
pub trait Tuple18MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> {
    #[doc = "Mapping all item for Tuple18"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> Tuple18MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17))
    }
}
#[doc = "Mapping for Tuple19"]
pub trait Tuple19Map<T> {
    #[doc = "Mapping for Tuple19"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple19Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18))
    }
}
#[doc = "Mapping all item for Tuple19"]
pub trait Tuple19MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> {
    #[doc = "Mapping all item for Tuple19"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> Tuple19MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18))
    }
}
#[doc = "Mapping for Tuple20"]
pub trait Tuple20Map<T> {
    #[doc = "Mapping for Tuple20"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple20Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19))
    }
}
#[doc = "Mapping all item for Tuple20"]
pub trait Tuple20MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> {
    #[doc = "Mapping all item for Tuple20"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> Tuple20MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19))
    }
}
#[doc = "Mapping for Tuple21"]
pub trait Tuple21Map<T> {
    #[doc = "Mapping for Tuple21"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple21Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20))
    }
}
#[doc = "Mapping all item for Tuple21"]
pub trait Tuple21MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> {
    #[doc = "Mapping all item for Tuple21"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> Tuple21MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20))
    }
}
#[doc = "Mapping for Tuple22"]
pub trait Tuple22Map<T> {
    #[doc = "Mapping for Tuple22"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple22Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21))
    }
}
#[doc = "Mapping all item for Tuple22"]
pub trait Tuple22MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> {
    #[doc = "Mapping all item for Tuple22"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> Tuple22MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21))
    }
}
#[doc = "Mapping for Tuple23"]
pub trait Tuple23Map<T> {
    #[doc = "Mapping for Tuple23"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple23Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22))
    }
}
#[doc = "Mapping all item for Tuple23"]
pub trait Tuple23MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> {
    #[doc = "Mapping all item for Tuple23"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> Tuple23MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22))
    }
}
#[doc = "Mapping for Tuple24"]
pub trait Tuple24Map<T> {
    #[doc = "Mapping for Tuple24"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple24Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23))
    }
}
#[doc = "Mapping all item for Tuple24"]
pub trait Tuple24MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> {
    #[doc = "Mapping all item for Tuple24"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> Tuple24MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23))
    }
}
#[doc = "Mapping for Tuple25"]
pub trait Tuple25Map<T> {
    #[doc = "Mapping for Tuple25"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple25Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24))
    }
}
#[doc = "Mapping all item for Tuple25"]
pub trait Tuple25MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> {
    #[doc = "Mapping all item for Tuple25"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> Tuple25MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24))
    }
}
#[doc = "Mapping for Tuple26"]
pub trait Tuple26Map<T> {
    #[doc = "Mapping for Tuple26"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple26Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25))
    }
}
#[doc = "Mapping all item for Tuple26"]
pub trait Tuple26MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> {
    #[doc = "Mapping all item for Tuple26"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> Tuple26MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25))
    }
}
#[doc = "Mapping for Tuple27"]
pub trait Tuple27Map<T> {
    #[doc = "Mapping for Tuple27"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple27Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25), f(self.26))
    }
}
#[doc = "Mapping all item for Tuple27"]
pub trait Tuple27MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> {
    #[doc = "Mapping all item for Tuple27"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25, f26: impl FnMut(T26) -> U26) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> Tuple27MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25, mut f26: impl FnMut(T26) -> U26) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25), f26(self.26))
    }
}
#[doc = "Mapping for Tuple28"]
pub trait Tuple28Map<T> {
    #[doc = "Mapping for Tuple28"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple28Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25), f(self.26), f(self.27))
    }
}
#[doc = "Mapping all item for Tuple28"]
pub trait Tuple28MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> {
    #[doc = "Mapping all item for Tuple28"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25, f26: impl FnMut(T26) -> U26, f27: impl FnMut(T27) -> U27) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> Tuple28MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25, mut f26: impl FnMut(T26) -> U26, mut f27: impl FnMut(T27) -> U27) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25), f26(self.26), f27(self.27))
    }
}
#[doc = "Mapping for Tuple29"]
pub trait Tuple29Map<T> {
    #[doc = "Mapping for Tuple29"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple29Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25), f(self.26), f(self.27), f(self.28))
    }
}
#[doc = "Mapping all item for Tuple29"]
pub trait Tuple29MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> {
    #[doc = "Mapping all item for Tuple29"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25, f26: impl FnMut(T26) -> U26, f27: impl FnMut(T27) -> U27, f28: impl FnMut(T28) -> U28) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> Tuple29MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25, mut f26: impl FnMut(T26) -> U26, mut f27: impl FnMut(T27) -> U27, mut f28: impl FnMut(T28) -> U28) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25), f26(self.26), f27(self.27), f28(self.28))
    }
}
#[doc = "Mapping for Tuple30"]
pub trait Tuple30Map<T> {
    #[doc = "Mapping for Tuple30"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple30Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25), f(self.26), f(self.27), f(self.28), f(self.29))
    }
}
#[doc = "Mapping all item for Tuple30"]
pub trait Tuple30MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> {
    #[doc = "Mapping all item for Tuple30"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25, f26: impl FnMut(T26) -> U26, f27: impl FnMut(T27) -> U27, f28: impl FnMut(T28) -> U28, f29: impl FnMut(T29) -> U29) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> Tuple30MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25, mut f26: impl FnMut(T26) -> U26, mut f27: impl FnMut(T27) -> U27, mut f28: impl FnMut(T28) -> U28, mut f29: impl FnMut(T29) -> U29) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25), f26(self.26), f27(self.27), f28(self.28), f29(self.29))
    }
}
#[doc = "Mapping for Tuple31"]
pub trait Tuple31Map<T> {
    #[doc = "Mapping for Tuple31"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple31Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25), f(self.26), f(self.27), f(self.28), f(self.29), f(self.30))
    }
}
#[doc = "Mapping all item for Tuple31"]
pub trait Tuple31MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> {
    #[doc = "Mapping all item for Tuple31"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25, f26: impl FnMut(T26) -> U26, f27: impl FnMut(T27) -> U27, f28: impl FnMut(T28) -> U28, f29: impl FnMut(T29) -> U29, f30: impl FnMut(T30) -> U30) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> Tuple31MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25, mut f26: impl FnMut(T26) -> U26, mut f27: impl FnMut(T27) -> U27, mut f28: impl FnMut(T28) -> U28, mut f29: impl FnMut(T29) -> U29, mut f30: impl FnMut(T30) -> U30) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25), f26(self.26), f27(self.27), f28(self.28), f29(self.29), f30(self.30))
    }
}
#[doc = "Mapping for Tuple32"]
pub trait Tuple32Map<T> {
    #[doc = "Mapping for Tuple32"]
    fn map<U>(self, f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U);
}
impl<T> Tuple32Map<T> for (T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T) {
    fn map<U>(self, mut f: impl FnMut(T) -> U) -> (U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U) {
        (f(self.0), f(self.1), f(self.2), f(self.3), f(self.4), f(self.5), f(self.6), f(self.7), f(self.8), f(self.9), f(self.10), f(self.11), f(self.12), f(self.13), f(self.14), f(self.15), f(self.16), f(self.17), f(self.18), f(self.19), f(self.20), f(self.21), f(self.22), f(self.23), f(self.24), f(self.25), f(self.26), f(self.27), f(self.28), f(self.29), f(self.30), f(self.31))
    }
}
#[doc = "Mapping all item for Tuple32"]
pub trait Tuple32MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> {
    #[doc = "Mapping all item for Tuple32"]
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31>(self, f0: impl FnMut(T0) -> U0, f1: impl FnMut(T1) -> U1, f2: impl FnMut(T2) -> U2, f3: impl FnMut(T3) -> U3, f4: impl FnMut(T4) -> U4, f5: impl FnMut(T5) -> U5, f6: impl FnMut(T6) -> U6, f7: impl FnMut(T7) -> U7, f8: impl FnMut(T8) -> U8, f9: impl FnMut(T9) -> U9, f10: impl FnMut(T10) -> U10, f11: impl FnMut(T11) -> U11, f12: impl FnMut(T12) -> U12, f13: impl FnMut(T13) -> U13, f14: impl FnMut(T14) -> U14, f15: impl FnMut(T15) -> U15, f16: impl FnMut(T16) -> U16, f17: impl FnMut(T17) -> U17, f18: impl FnMut(T18) -> U18, f19: impl FnMut(T19) -> U19, f20: impl FnMut(T20) -> U20, f21: impl FnMut(T21) -> U21, f22: impl FnMut(T22) -> U22, f23: impl FnMut(T23) -> U23, f24: impl FnMut(T24) -> U24, f25: impl FnMut(T25) -> U25, f26: impl FnMut(T26) -> U26, f27: impl FnMut(T27) -> U27, f28: impl FnMut(T28) -> U28, f29: impl FnMut(T29) -> U29, f30: impl FnMut(T30) -> U30, f31: impl FnMut(T31) -> U31) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31);
}
impl<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> Tuple32MapAll<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) {
    fn map_all<U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31>(self, mut f0: impl FnMut(T0) -> U0, mut f1: impl FnMut(T1) -> U1, mut f2: impl FnMut(T2) -> U2, mut f3: impl FnMut(T3) -> U3, mut f4: impl FnMut(T4) -> U4, mut f5: impl FnMut(T5) -> U5, mut f6: impl FnMut(T6) -> U6, mut f7: impl FnMut(T7) -> U7, mut f8: impl FnMut(T8) -> U8, mut f9: impl FnMut(T9) -> U9, mut f10: impl FnMut(T10) -> U10, mut f11: impl FnMut(T11) -> U11, mut f12: impl FnMut(T12) -> U12, mut f13: impl FnMut(T13) -> U13, mut f14: impl FnMut(T14) -> U14, mut f15: impl FnMut(T15) -> U15, mut f16: impl FnMut(T16) -> U16, mut f17: impl FnMut(T17) -> U17, mut f18: impl FnMut(T18) -> U18, mut f19: impl FnMut(T19) -> U19, mut f20: impl FnMut(T20) -> U20, mut f21: impl FnMut(T21) -> U21, mut f22: impl FnMut(T22) -> U22, mut f23: impl FnMut(T23) -> U23, mut f24: impl FnMut(T24) -> U24, mut f25: impl FnMut(T25) -> U25, mut f26: impl FnMut(T26) -> U26, mut f27: impl FnMut(T27) -> U27, mut f28: impl FnMut(T28) -> U28, mut f29: impl FnMut(T29) -> U29, mut f30: impl FnMut(T30) -> U30, mut f31: impl FnMut(T31) -> U31) -> (U0, U1, U2, U3, U4, U5, U6, U7, U8, U9, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U20, U21, U22, U23, U24, U25, U26, U27, U28, U29, U30, U31) {
        (f0(self.0), f1(self.1), f2(self.2), f3(self.3), f4(self.4), f5(self.5), f6(self.6), f7(self.7), f8(self.8), f9(self.9), f10(self.10), f11(self.11), f12(self.12), f13(self.13), f14(self.14), f15(self.15), f16(self.16), f17(self.17), f18(self.18), f19(self.19), f20(self.20), f21(self.21), f22(self.22), f23(self.23), f24(self.24), f25(self.25), f26(self.26), f27(self.27), f28(self.28), f29(self.29), f30(self.30), f31(self.31))
    }
}
