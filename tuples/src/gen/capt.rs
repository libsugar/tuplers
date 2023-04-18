// This file is by code gen, do not modify

#[doc = "Manually capture the variables required by the closure"]
pub fn capt_1<C, T0, R, F: Fn(&C, T0) -> R>(c: C, f: F) -> impl Fn(T0) -> R {
    move |v0| f(&c, v0)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_1<C, T0, R, F: FnMut(&mut C, T0) -> R>(mut c: C, mut f: F) -> impl FnMut(T0) -> R {
    move |v0| f(&mut c, v0)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_1<C, T0, R, F: FnOnce(C, T0) -> R>(c: C, f: F) -> impl FnOnce(T0) -> R {
    move |v0| f(c, v0)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_2<C, T0, T1, R, F: Fn(&C, T0, T1) -> R>(c: C, f: F) -> impl Fn(T0, T1) -> R {
    move |v0, v1| f(&c, v0, v1)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_2<C, T0, T1, R, F: FnMut(&mut C, T0, T1) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1) -> R {
    move |v0, v1| f(&mut c, v0, v1)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_2<C, T0, T1, R, F: FnOnce(C, T0, T1) -> R>(c: C, f: F) -> impl FnOnce(T0, T1) -> R {
    move |v0, v1| f(c, v0, v1)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_3<C, T0, T1, T2, R, F: Fn(&C, T0, T1, T2) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2) -> R {
    move |v0, v1, v2| f(&c, v0, v1, v2)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_3<C, T0, T1, T2, R, F: FnMut(&mut C, T0, T1, T2) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2) -> R {
    move |v0, v1, v2| f(&mut c, v0, v1, v2)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_3<C, T0, T1, T2, R, F: FnOnce(C, T0, T1, T2) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2) -> R {
    move |v0, v1, v2| f(c, v0, v1, v2)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_4<C, T0, T1, T2, T3, R, F: Fn(&C, T0, T1, T2, T3) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3) -> R {
    move |v0, v1, v2, v3| f(&c, v0, v1, v2, v3)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_4<C, T0, T1, T2, T3, R, F: FnMut(&mut C, T0, T1, T2, T3) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3) -> R {
    move |v0, v1, v2, v3| f(&mut c, v0, v1, v2, v3)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_4<C, T0, T1, T2, T3, R, F: FnOnce(C, T0, T1, T2, T3) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3) -> R {
    move |v0, v1, v2, v3| f(c, v0, v1, v2, v3)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_5<C, T0, T1, T2, T3, T4, R, F: Fn(&C, T0, T1, T2, T3, T4) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4) -> R {
    move |v0, v1, v2, v3, v4| f(&c, v0, v1, v2, v3, v4)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_5<C, T0, T1, T2, T3, T4, R, F: FnMut(&mut C, T0, T1, T2, T3, T4) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4) -> R {
    move |v0, v1, v2, v3, v4| f(&mut c, v0, v1, v2, v3, v4)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_5<C, T0, T1, T2, T3, T4, R, F: FnOnce(C, T0, T1, T2, T3, T4) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4) -> R {
    move |v0, v1, v2, v3, v4| f(c, v0, v1, v2, v3, v4)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_6<C, T0, T1, T2, T3, T4, T5, R, F: Fn(&C, T0, T1, T2, T3, T4, T5) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5) -> R {
    move |v0, v1, v2, v3, v4, v5| f(&c, v0, v1, v2, v3, v4, v5)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_6<C, T0, T1, T2, T3, T4, T5, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5) -> R {
    move |v0, v1, v2, v3, v4, v5| f(&mut c, v0, v1, v2, v3, v4, v5)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_6<C, T0, T1, T2, T3, T4, T5, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5) -> R {
    move |v0, v1, v2, v3, v4, v5| f(c, v0, v1, v2, v3, v4, v5)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_7<C, T0, T1, T2, T3, T4, T5, T6, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6) -> R {
    move |v0, v1, v2, v3, v4, v5, v6| f(&c, v0, v1, v2, v3, v4, v5, v6)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_7<C, T0, T1, T2, T3, T4, T5, T6, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6) -> R {
    move |v0, v1, v2, v3, v4, v5, v6| f(&mut c, v0, v1, v2, v3, v4, v5, v6)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_7<C, T0, T1, T2, T3, T4, T5, T6, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6) -> R {
    move |v0, v1, v2, v3, v4, v5, v6| f(c, v0, v1, v2, v3, v4, v5, v6)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_8<C, T0, T1, T2, T3, T4, T5, T6, T7, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7| f(&c, v0, v1, v2, v3, v4, v5, v6, v7)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_8<C, T0, T1, T2, T3, T4, T5, T6, T7, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_8<C, T0, T1, T2, T3, T4, T5, T6, T7, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7| f(c, v0, v1, v2, v3, v4, v5, v6, v7)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_9<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_9<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_9<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_10<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_10<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_10<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_11<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_11<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_11<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_12<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_12<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_12<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_13<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_13<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_13<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_14<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_14<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_14<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_15<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_15<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_15<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_16<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_16<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_16<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_17<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_17<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_17<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_18<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_18<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_18<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_19<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_19<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_19<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_20<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_20<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_20<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_21<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_21<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_21<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_22<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_22<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_22<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_23<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_23<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_23<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_24<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_24<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_24<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_25<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_25<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_25<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_26<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_26<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_26<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_27<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_27<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_27<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_28<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_28<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_28<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_29<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_29<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_29<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_30<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_30<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_30<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_31<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_31<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_31<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_32<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, R, F: Fn(&C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R>(c: C, f: F) -> impl Fn(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31| f(&c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_mut_32<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, R, F: FnMut(&mut C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R>(mut c: C, mut f: F) -> impl FnMut(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31| f(&mut c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
}
#[doc = "Manually capture the variables required by the closure"]
pub fn capt_once_32<C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, R, F: FnOnce(C, T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R>(c: C, f: F) -> impl FnOnce(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20, T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31) -> R {
    move |v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31| f(c, v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31)
}
