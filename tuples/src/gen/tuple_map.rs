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
