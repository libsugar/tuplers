// auto generated code, do not modify

pub trait AFnOnce0: FnOnce() -> Self::Ret {
    type Ret;
}
pub trait AFnMut0: AFnOnce0 + FnMut() -> Self::Ret {}
pub trait AFn0: AFnMut0 + Fn() -> Self::Ret {}
impl<F, R> AFnOnce0 for F
where
    F: FnOnce() -> R,
{
    type Ret = R;
}
impl<F, R> AFnMut0 for F where F: FnMut() -> R {}
impl<F, R> AFn0 for F where F: Fn() -> R {}
pub trait FuFnOnce0: FnOnce() -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut0: FuFnOnce0 + FnMut() -> Self::Future {}
pub trait FuFn0: FuFnMut0 + Fn() -> Self::Future {}
impl<F, Fu, R> FuFnOnce0 for F
where
    F: FnOnce() -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R> FuFnMut0 for F
where
    F: FnMut() -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R> FuFn0 for F
where
    F: Fn() -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce1<T0>: FnOnce(T0) -> Self::Ret {
    type Ret;
}
pub trait AFnMut1<T0>: AFnOnce1<T0> + FnMut(T0) -> Self::Ret {}
pub trait AFn1<T0>: AFnMut1<T0> + Fn(T0) -> Self::Ret {}
impl<F, R, T0> AFnOnce1<T0> for F
where
    F: FnOnce(T0) -> R,
{
    type Ret = R;
}
impl<F, R, T0> AFnMut1<T0> for F where F: FnMut(T0) -> R {}
impl<F, R, T0> AFn1<T0> for F where F: Fn(T0) -> R {}
pub trait FuFnOnce1<T0>: FnOnce(T0) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut1<T0>: FuFnOnce1<T0> + FnMut(T0) -> Self::Future {}
pub trait FuFn1<T0>: FuFnMut1<T0> + Fn(T0) -> Self::Future {}
impl<F, Fu, R, T0> FuFnOnce1<T0> for F
where
    F: FnOnce(T0) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0> FuFnMut1<T0> for F
where
    F: FnMut(T0) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0> FuFn1<T0> for F
where
    F: Fn(T0) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce2<T0, T1>: FnOnce(T0, T1) -> Self::Ret {
    type Ret;
}
pub trait AFnMut2<T0, T1>: AFnOnce2<T0, T1> + FnMut(T0, T1) -> Self::Ret {}
pub trait AFn2<T0, T1>: AFnMut2<T0, T1> + Fn(T0, T1) -> Self::Ret {}
impl<F, R, T0, T1> AFnOnce2<T0, T1> for F
where
    F: FnOnce(T0, T1) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1> AFnMut2<T0, T1> for F where F: FnMut(T0, T1) -> R {}
impl<F, R, T0, T1> AFn2<T0, T1> for F where F: Fn(T0, T1) -> R {}
pub trait FuFnOnce2<T0, T1>: FnOnce(T0, T1) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut2<T0, T1>: FuFnOnce2<T0, T1> + FnMut(T0, T1) -> Self::Future {}
pub trait FuFn2<T0, T1>: FuFnMut2<T0, T1> + Fn(T0, T1) -> Self::Future {}
impl<F, Fu, R, T0, T1> FuFnOnce2<T0, T1> for F
where
    F: FnOnce(T0, T1) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1> FuFnMut2<T0, T1> for F
where
    F: FnMut(T0, T1) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1> FuFn2<T0, T1> for F
where
    F: Fn(T0, T1) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce3<T0, T1, T2>: FnOnce(T0, T1, T2) -> Self::Ret {
    type Ret;
}
pub trait AFnMut3<T0, T1, T2>: AFnOnce3<T0, T1, T2> + FnMut(T0, T1, T2) -> Self::Ret {}
pub trait AFn3<T0, T1, T2>: AFnMut3<T0, T1, T2> + Fn(T0, T1, T2) -> Self::Ret {}
impl<F, R, T0, T1, T2> AFnOnce3<T0, T1, T2> for F
where
    F: FnOnce(T0, T1, T2) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2> AFnMut3<T0, T1, T2> for F where F: FnMut(T0, T1, T2) -> R {}
impl<F, R, T0, T1, T2> AFn3<T0, T1, T2> for F where F: Fn(T0, T1, T2) -> R {}
pub trait FuFnOnce3<T0, T1, T2>: FnOnce(T0, T1, T2) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut3<T0, T1, T2>: FuFnOnce3<T0, T1, T2> + FnMut(T0, T1, T2) -> Self::Future {}
pub trait FuFn3<T0, T1, T2>: FuFnMut3<T0, T1, T2> + Fn(T0, T1, T2) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2> FuFnOnce3<T0, T1, T2> for F
where
    F: FnOnce(T0, T1, T2) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2> FuFnMut3<T0, T1, T2> for F
where
    F: FnMut(T0, T1, T2) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2> FuFn3<T0, T1, T2> for F
where
    F: Fn(T0, T1, T2) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce4<T0, T1, T2, T3>: FnOnce(T0, T1, T2, T3) -> Self::Ret {
    type Ret;
}
pub trait AFnMut4<T0, T1, T2, T3>: AFnOnce4<T0, T1, T2, T3> + FnMut(T0, T1, T2, T3) -> Self::Ret {}
pub trait AFn4<T0, T1, T2, T3>: AFnMut4<T0, T1, T2, T3> + Fn(T0, T1, T2, T3) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3> AFnOnce4<T0, T1, T2, T3> for F
where
    F: FnOnce(T0, T1, T2, T3) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3> AFnMut4<T0, T1, T2, T3> for F where F: FnMut(T0, T1, T2, T3) -> R {}
impl<F, R, T0, T1, T2, T3> AFn4<T0, T1, T2, T3> for F where F: Fn(T0, T1, T2, T3) -> R {}
pub trait FuFnOnce4<T0, T1, T2, T3>: FnOnce(T0, T1, T2, T3) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut4<T0, T1, T2, T3>: FuFnOnce4<T0, T1, T2, T3> + FnMut(T0, T1, T2, T3) -> Self::Future {}
pub trait FuFn4<T0, T1, T2, T3>: FuFnMut4<T0, T1, T2, T3> + Fn(T0, T1, T2, T3) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3> FuFnOnce4<T0, T1, T2, T3> for F
where
    F: FnOnce(T0, T1, T2, T3) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3> FuFnMut4<T0, T1, T2, T3> for F
where
    F: FnMut(T0, T1, T2, T3) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3> FuFn4<T0, T1, T2, T3> for F
where
    F: Fn(T0, T1, T2, T3) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce5<T0, T1, T2, T3, T4>: FnOnce(T0, T1, T2, T3, T4) -> Self::Ret {
    type Ret;
}
pub trait AFnMut5<T0, T1, T2, T3, T4>: AFnOnce5<T0, T1, T2, T3, T4> + FnMut(T0, T1, T2, T3, T4) -> Self::Ret {}
pub trait AFn5<T0, T1, T2, T3, T4>: AFnMut5<T0, T1, T2, T3, T4> + Fn(T0, T1, T2, T3, T4) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4> AFnOnce5<T0, T1, T2, T3, T4> for F
where
    F: FnOnce(T0, T1, T2, T3, T4) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4> AFnMut5<T0, T1, T2, T3, T4> for F where F: FnMut(T0, T1, T2, T3, T4) -> R {}
impl<F, R, T0, T1, T2, T3, T4> AFn5<T0, T1, T2, T3, T4> for F where F: Fn(T0, T1, T2, T3, T4) -> R {}
pub trait FuFnOnce5<T0, T1, T2, T3, T4>: FnOnce(T0, T1, T2, T3, T4) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut5<T0, T1, T2, T3, T4>: FuFnOnce5<T0, T1, T2, T3, T4> + FnMut(T0, T1, T2, T3, T4) -> Self::Future {}
pub trait FuFn5<T0, T1, T2, T3, T4>: FuFnMut5<T0, T1, T2, T3, T4> + Fn(T0, T1, T2, T3, T4) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4> FuFnOnce5<T0, T1, T2, T3, T4> for F
where
    F: FnOnce(T0, T1, T2, T3, T4) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4> FuFnMut5<T0, T1, T2, T3, T4> for F
where
    F: FnMut(T0, T1, T2, T3, T4) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4> FuFn5<T0, T1, T2, T3, T4> for F
where
    F: Fn(T0, T1, T2, T3, T4) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce6<T0, T1, T2, T3, T4, T5>: FnOnce(T0, T1, T2, T3, T4, T5) -> Self::Ret {
    type Ret;
}
pub trait AFnMut6<T0, T1, T2, T3, T4, T5>: AFnOnce6<T0, T1, T2, T3, T4, T5> + FnMut(T0, T1, T2, T3, T4, T5) -> Self::Ret {}
pub trait AFn6<T0, T1, T2, T3, T4, T5>: AFnMut6<T0, T1, T2, T3, T4, T5> + Fn(T0, T1, T2, T3, T4, T5) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5> AFnOnce6<T0, T1, T2, T3, T4, T5> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5> AFnMut6<T0, T1, T2, T3, T4, T5> for F where F: FnMut(T0, T1, T2, T3, T4, T5) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5> AFn6<T0, T1, T2, T3, T4, T5> for F where F: Fn(T0, T1, T2, T3, T4, T5) -> R {}
pub trait FuFnOnce6<T0, T1, T2, T3, T4, T5>: FnOnce(T0, T1, T2, T3, T4, T5) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut6<T0, T1, T2, T3, T4, T5>: FuFnOnce6<T0, T1, T2, T3, T4, T5> + FnMut(T0, T1, T2, T3, T4, T5) -> Self::Future {}
pub trait FuFn6<T0, T1, T2, T3, T4, T5>: FuFnMut6<T0, T1, T2, T3, T4, T5> + Fn(T0, T1, T2, T3, T4, T5) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5> FuFnOnce6<T0, T1, T2, T3, T4, T5> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5> FuFnMut6<T0, T1, T2, T3, T4, T5> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5> FuFn6<T0, T1, T2, T3, T4, T5> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce7<T0, T1, T2, T3, T4, T5, T6>: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> Self::Ret {
    type Ret;
}
pub trait AFnMut7<T0, T1, T2, T3, T4, T5, T6>: AFnOnce7<T0, T1, T2, T3, T4, T5, T6> + FnMut(T0, T1, T2, T3, T4, T5, T6) -> Self::Ret {}
pub trait AFn7<T0, T1, T2, T3, T4, T5, T6>: AFnMut7<T0, T1, T2, T3, T4, T5, T6> + Fn(T0, T1, T2, T3, T4, T5, T6) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6> AFnOnce7<T0, T1, T2, T3, T4, T5, T6> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6> AFnMut7<T0, T1, T2, T3, T4, T5, T6> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6> AFn7<T0, T1, T2, T3, T4, T5, T6> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6) -> R {}
pub trait FuFnOnce7<T0, T1, T2, T3, T4, T5, T6>: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut7<T0, T1, T2, T3, T4, T5, T6>: FuFnOnce7<T0, T1, T2, T3, T4, T5, T6> + FnMut(T0, T1, T2, T3, T4, T5, T6) -> Self::Future {}
pub trait FuFn7<T0, T1, T2, T3, T4, T5, T6>: FuFnMut7<T0, T1, T2, T3, T4, T5, T6> + Fn(T0, T1, T2, T3, T4, T5, T6) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6> FuFnOnce7<T0, T1, T2, T3, T4, T5, T6> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6> FuFnMut7<T0, T1, T2, T3, T4, T5, T6> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6> FuFn7<T0, T1, T2, T3, T4, T5, T6> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce8<T0, T1, T2, T3, T4, T5, T6, T7>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> Self::Ret {
    type Ret;
}
pub trait AFnMut8<T0, T1, T2, T3, T4, T5, T6, T7>: AFnOnce8<T0, T1, T2, T3, T4, T5, T6, T7> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7) -> Self::Ret {}
pub trait AFn8<T0, T1, T2, T3, T4, T5, T6, T7>: AFnMut8<T0, T1, T2, T3, T4, T5, T6, T7> + Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7> AFnOnce8<T0, T1, T2, T3, T4, T5, T6, T7> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7> AFnMut8<T0, T1, T2, T3, T4, T5, T6, T7> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7> AFn8<T0, T1, T2, T3, T4, T5, T6, T7> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R {}
pub trait FuFnOnce8<T0, T1, T2, T3, T4, T5, T6, T7>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut8<T0, T1, T2, T3, T4, T5, T6, T7>: FuFnOnce8<T0, T1, T2, T3, T4, T5, T6, T7> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7) -> Self::Future {}
pub trait FuFn8<T0, T1, T2, T3, T4, T5, T6, T7>: FuFnMut8<T0, T1, T2, T3, T4, T5, T6, T7> + Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7> FuFnOnce8<T0, T1, T2, T3, T4, T5, T6, T7> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7> FuFnMut8<T0, T1, T2, T3, T4, T5, T6, T7> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7> FuFn8<T0, T1, T2, T3, T4, T5, T6, T7> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce9<T0, T1, T2, T3, T4, T5, T6, T7, T8>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Self::Ret {
    type Ret;
}
pub trait AFnMut9<T0, T1, T2, T3, T4, T5, T6, T7, T8>: AFnOnce9<T0, T1, T2, T3, T4, T5, T6, T7, T8> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Self::Ret {}
pub trait AFn9<T0, T1, T2, T3, T4, T5, T6, T7, T8>: AFnMut9<T0, T1, T2, T3, T4, T5, T6, T7, T8> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> AFnOnce9<T0, T1, T2, T3, T4, T5, T6, T7, T8> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> AFnMut9<T0, T1, T2, T3, T4, T5, T6, T7, T8> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> AFn9<T0, T1, T2, T3, T4, T5, T6, T7, T8> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R {}
pub trait FuFnOnce9<T0, T1, T2, T3, T4, T5, T6, T7, T8>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut9<T0, T1, T2, T3, T4, T5, T6, T7, T8>: FuFnOnce9<T0, T1, T2, T3, T4, T5, T6, T7, T8> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Self::Future {}
pub trait FuFn9<T0, T1, T2, T3, T4, T5, T6, T7, T8>: FuFnMut9<T0, T1, T2, T3, T4, T5, T6, T7, T8> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> FuFnOnce9<T0, T1, T2, T3, T4, T5, T6, T7, T8> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> FuFnMut9<T0, T1, T2, T3, T4, T5, T6, T7, T8> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8> FuFn9<T0, T1, T2, T3, T4, T5, T6, T7, T8> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Self::Ret {
    type Ret;
}
pub trait AFnMut10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>: AFnOnce10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Self::Ret {}
pub trait AFn10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>: AFnMut10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> AFnOnce10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> AFnMut10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> AFn10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R {}
pub trait FuFnOnce10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>: FuFnOnce10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Self::Future {}
pub trait FuFn10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9>: FuFnMut10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> FuFnOnce10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> FuFnMut10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> FuFn10<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Self::Ret {
    type Ret;
}
pub trait AFnMut11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: AFnOnce11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Self::Ret {}
pub trait AFn11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: AFnMut11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> AFnOnce11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> AFnMut11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> AFn11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R {}
pub trait FuFnOnce11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: FuFnOnce11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Self::Future {}
pub trait FuFn11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>: FuFnMut11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> FuFnOnce11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> FuFnMut11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> FuFn11<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Self::Ret {
    type Ret;
}
pub trait AFnMut12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: AFnOnce12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Self::Ret {}
pub trait AFn12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: AFnMut12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AFnOnce12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AFnMut12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> AFn12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R {}
pub trait FuFnOnce12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: FuFnOnce12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Self::Future {}
pub trait FuFn12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>: FuFnMut12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> FuFnOnce12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> FuFnMut12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> FuFn12<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Self::Ret {
    type Ret;
}
pub trait AFnMut13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: AFnOnce13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Self::Ret {}
pub trait AFn13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: AFnMut13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> AFnOnce13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> AFnMut13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> AFn13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R {}
pub trait FuFnOnce13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: FuFnOnce13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Self::Future {}
pub trait FuFn13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>: FuFnMut13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> FuFnOnce13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> FuFnMut13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> FuFn13<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Self::Ret {
    type Ret;
}
pub trait AFnMut14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>: AFnOnce14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Self::Ret {}
pub trait AFn14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>: AFnMut14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> AFnOnce14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> AFnMut14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> AFn14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R {}
pub trait FuFnOnce14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>: FuFnOnce14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Self::Future {}
pub trait FuFn14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>: FuFnMut14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> FuFnOnce14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> FuFnMut14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> FuFn14<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Self::Ret {
    type Ret;
}
pub trait AFnMut15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>: AFnOnce15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Self::Ret {}
pub trait AFn15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>: AFnMut15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> AFnOnce15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> AFnMut15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> AFn15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R {}
pub trait FuFnOnce15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>: FuFnOnce15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Self::Future {}
pub trait FuFn15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>: FuFnMut15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> FuFnOnce15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> FuFnMut15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> FuFn15<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Self::Ret {
    type Ret;
}
pub trait AFnMut16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>: AFnOnce16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Self::Ret {}
pub trait AFn16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>: AFnMut16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> AFnOnce16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> AFnMut16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> AFn16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R {}
pub trait FuFnOnce16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>: FuFnOnce16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Self::Future {}
pub trait FuFn16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>: FuFnMut16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> FuFnOnce16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> FuFnMut16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> FuFn16<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Self::Ret {
    type Ret;
}
pub trait AFnMut17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>: AFnOnce17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Self::Ret {}
pub trait AFn17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>: AFnMut17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> AFnOnce17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> AFnMut17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> AFn17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R {}
pub trait FuFnOnce17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>: FuFnOnce17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Self::Future {}
pub trait FuFn17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>: FuFnMut17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> FuFnOnce17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> FuFnMut17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> FuFn17<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Self::Ret {
    type Ret;
}
pub trait AFnMut18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>: AFnOnce18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Self::Ret {}
pub trait AFn18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>: AFnMut18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> AFnOnce18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> AFnMut18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> AFn18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R {}
pub trait FuFnOnce18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>: FuFnOnce18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Self::Future {}
pub trait FuFn18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>: FuFnMut18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> FuFnOnce18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> FuFnMut18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> FuFn18<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Self::Ret {
    type Ret;
}
pub trait AFnMut19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>: AFnOnce19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Self::Ret {}
pub trait AFn19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>: AFnMut19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> AFnOnce19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> AFnMut19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> AFn19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R {}
pub trait FuFnOnce19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>: FuFnOnce19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Self::Future {}
pub trait FuFn19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>: FuFnMut19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> FuFnOnce19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> FuFnMut19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> FuFn19<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Self::Ret {
    type Ret;
}
pub trait AFnMut20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>: AFnOnce20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Self::Ret {}
pub trait AFn20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>: AFnMut20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> AFnOnce20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> AFnMut20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> AFn20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R {}
pub trait FuFnOnce20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>: FuFnOnce20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Self::Future {}
pub trait FuFn20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>: FuFnMut20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> FuFnOnce20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> FuFnMut20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> FuFn20<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Self::Ret {
    type Ret;
}
pub trait AFnMut21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>: AFnOnce21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Self::Ret {}
pub trait AFn21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>: AFnMut21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> AFnOnce21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> AFnMut21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> AFn21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R {}
pub trait FuFnOnce21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>: FuFnOnce21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Self::Future {}
pub trait FuFn21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20>: FuFnMut21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> FuFnOnce21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> FuFnMut21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> FuFn21<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Self::Ret {
    type Ret;
}
pub trait AFnMut22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>: AFnOnce22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Self::Ret {}
pub trait AFn22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>: AFnMut22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> AFnOnce22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> AFnMut22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> AFn22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R {}
pub trait FuFnOnce22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>: FuFnOnce22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Self::Future {}
pub trait FuFn22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21>: FuFnMut22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> FuFnOnce22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> FuFnMut22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> FuFn22<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Self::Ret {
    type Ret;
}
pub trait AFnMut23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>: AFnOnce23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Self::Ret {}
pub trait AFn23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>: AFnMut23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> AFnOnce23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> AFnMut23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> AFn23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R {}
pub trait FuFnOnce23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>: FuFnOnce23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Self::Future {}
pub trait FuFn23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22>: FuFnMut23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> FuFnOnce23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> FuFnMut23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> FuFn23<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Self::Ret {
    type Ret;
}
pub trait AFnMut24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>: AFnOnce24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Self::Ret {}
pub trait AFn24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>: AFnMut24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> AFnOnce24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> AFnMut24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> AFn24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R {}
pub trait FuFnOnce24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>: FuFnOnce24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Self::Future {}
pub trait FuFn24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23>: FuFnMut24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> FuFnOnce24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> FuFnMut24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> FuFn24<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Self::Ret {
    type Ret;
}
pub trait AFnMut25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>: AFnOnce25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Self::Ret {}
pub trait AFn25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>: AFnMut25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> AFnOnce25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> AFnMut25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> AFn25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R {}
pub trait FuFnOnce25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>: FuFnOnce25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Self::Future {}
pub trait FuFn25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24>: FuFnMut25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> FuFnOnce25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> FuFnMut25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> FuFn25<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Self::Ret {
    type Ret;
}
pub trait AFnMut26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>: AFnOnce26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Self::Ret {}
pub trait AFn26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>: AFnMut26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> AFnOnce26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> AFnMut26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> AFn26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R {}
pub trait FuFnOnce26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>: FuFnOnce26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Self::Future {}
pub trait FuFn26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25>: FuFnMut26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> FuFnOnce26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> FuFnMut26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> FuFn26<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Self::Ret {
    type Ret;
}
pub trait AFnMut27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>: AFnOnce27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Self::Ret {}
pub trait AFn27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>: AFnMut27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> AFnOnce27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> AFnMut27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> AFn27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R {}
pub trait FuFnOnce27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>: FuFnOnce27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Self::Future {}
pub trait FuFn27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26>: FuFnMut27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> FuFnOnce27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> FuFnMut27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> FuFn27<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Self::Ret {
    type Ret;
}
pub trait AFnMut28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>: AFnOnce28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Self::Ret {}
pub trait AFn28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>: AFnMut28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> AFnOnce28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> AFnMut28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> AFn28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R {}
pub trait FuFnOnce28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>: FuFnOnce28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Self::Future {}
pub trait FuFn28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27>: FuFnMut28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> FuFnOnce28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> FuFnMut28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> FuFn28<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Self::Ret {
    type Ret;
}
pub trait AFnMut29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>: AFnOnce29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Self::Ret {}
pub trait AFn29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>: AFnMut29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> AFnOnce29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> AFnMut29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> AFn29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R {}
pub trait FuFnOnce29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>: FuFnOnce29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Self::Future {}
pub trait FuFn29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28>: FuFnMut29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> FuFnOnce29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> FuFnMut29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> FuFn29<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Self::Ret {
    type Ret;
}
pub trait AFnMut30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>: AFnOnce30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Self::Ret {}
pub trait AFn30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>: AFnMut30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> AFnOnce30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> AFnMut30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> AFn30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R {}
pub trait FuFnOnce30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>: FuFnOnce30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Self::Future {}
pub trait FuFn30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29>: FuFnMut30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> FuFnOnce30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> FuFnMut30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> FuFn30<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Self::Ret {
    type Ret;
}
pub trait AFnMut31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>: AFnOnce31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Self::Ret {}
pub trait AFn31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>: AFnMut31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> AFnOnce31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> AFnMut31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> AFn31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R {}
pub trait FuFnOnce31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>: FuFnOnce31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Self::Future {}
pub trait FuFn31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30>: FuFnMut31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> FuFnOnce31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> FuFnMut31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> FuFn31<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> Fu,
    Fu: Future<Output = R>,
{
}
pub trait AFnOnce32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Self::Ret {
    type Ret;
}
pub trait AFnMut32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>: AFnOnce32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Self::Ret {}
pub trait AFn32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>: AFnMut32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Self::Ret {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> AFnOnce32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R,
{
    type Ret = R;
}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> AFnMut32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for F where F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R {}
impl<F, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> AFn32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for F where F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R {}
pub trait FuFnOnce32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Self::Future {
    type Future: Future<Output = Self::Ret>;
    type Ret;
}
pub trait FuFnMut32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>: FuFnOnce32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> + FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Self::Future {}
pub trait FuFn32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31>: FuFnMut32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> + Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Self::Future {}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> FuFnOnce32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for F
where
    F: FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Fu,
    Fu: Future<Output = R>,
{
    type Future = Fu;
    type Ret = R;
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> FuFnMut32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for F
where
    F: FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Fu,
    Fu: Future<Output = R>,
{
}
impl<F, Fu, R, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> FuFn32<T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31> for F
where
    F: Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> Fu,
    Fu: Future<Output = R>,
{
}
