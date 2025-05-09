//! Making Fn trait have an associated type

use core::future::Future;

include!("./gen/afn.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use core::future::Future;

    async fn foo(_a: i32, b: &[f64]) -> &[f64] {
        b
    }

    async fn bar(b: &[f64], f: impl for<'b> FuFn2<i32, &'b [f64], Ret = &'b [f64]>) -> &[f64] {
        f(0, b).await
    }

    async fn bar2<'a>(b: &'a [f64], f: impl AFn2<i32, &'a [f64], Ret = impl Future<Output = &'a [f64]>>) -> &'a [f64] {
        f(0, b).await
    }

    #[allow(dead_code)]
    async fn test1(b: &[f64]) -> &[f64] {
        bar(b, foo).await
    }

    #[allow(dead_code)]
    async fn test2(b: &[f64]) -> &[f64] {
        // lambda can not have generic
        bar2(b, async |_a, b| b).await
    }

    #[allow(dead_code)]
    async fn test3(b: &[f64]) -> &[f64] {
        // lambda can not have generic
        bar2(b, |_a, b| async move { b }).await
    }

    #[allow(dead_code)]
    async fn test4(b: &[f64]) -> &[f64] {
        bar2(b, foo).await
    }
}
