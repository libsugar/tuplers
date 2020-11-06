use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::{ffi::OsString, fs, path::Path};
use syn::LitInt;

pub fn code_gen(out_dir: OsString) {
    let t = format_ident!("T");
    let u = format_ident!("U");

    let ctx = init(33, &t, &u);

    gen_tuple_impl(&ctx, &out_dir);
    gen_tuple_n_impl(&ctx, &out_dir);
    #[cfg(feature = "tuple_iter")]
    gen_tuple_iter(&ctx, &out_dir);
    #[cfg(feature = "tuple_map")]
    gen_tuple_map(&ctx, &out_dir);
}

struct Ctx<'a> {
    pub t: &'a Ident,
    pub u: &'a Ident,
    pub size_lits: Vec<LitInt>,
    pub ts: Vec<&'a Ident>,
    pub us: Vec<&'a Ident>,
    pub nts: Vec<Ident>,
}

fn init<'a>(max: usize, t: &'a Ident, u: &'a Ident) -> Ctx<'a> {
    let size_lits = (0..max + 1)
        .into_iter()
        .map(|i| LitInt::new(i.to_string().as_str(), Span::call_site()))
        .collect::<Vec<_>>();
    let ts = (0..max + 1).into_iter().map(|_| t).collect::<Vec<_>>();
    let us = (0..max + 1).into_iter().map(|_| u).collect::<Vec<_>>();
    let nts = (0..max + 1)
        .into_iter()
        .map(|i| format_ident!("T{}", i))
        .collect::<Vec<_>>();
    let ctx = Ctx {
        t,
        u,
        size_lits,
        ts,
        us,
        nts,
    };
    ctx
}

fn gen_tuple_impl(ctx: &Ctx, out_dir: &OsString) {
    let items = (2..33usize)
        .into_iter()
        .map(|i| gen_tuple_impl_size(ctx, i));
    let tks = quote! { #(#items)* };
    let code = tks.to_string();
    let dest_path = Path::new(out_dir).join("tuple_impl.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_tuple_impl_size(ctx: &Ctx, size: usize) -> TokenStream {
    let size_lit = &ctx.size_lits[size];

    let ts = &ctx.ts[0..size];

    let nts = &ctx.nts[0..size];
    let ants = (0..size)
        .into_iter()
        .map(|i| &nts[i])
        .map(|i| quote! { #i: 'a })
        .collect::<Vec<_>>();
    let ref_nts = (0..size)
        .into_iter()
        .map(|i| &nts[i])
        .map(|id| quote! { &'a #id })
        .collect::<Vec<_>>();
    let mut_nts = (0..size)
        .into_iter()
        .map(|i| &nts[i])
        .map(|id| quote! { &'a mut #id })
        .collect::<Vec<_>>();

    let ref_impl = ctx.size_lits[0..size].iter().map(|l| {
        quote! {
            &self.#l
        }
    });
    let mut_impl = ctx.size_lits[0..size].iter().map(|l| {
        quote! {
            &mut self.#l
        }
    });

    let ref_doc = format!("AsRef for Tuple{}", size);
    let mut_doc = format!("AsMut for Tuple{}", size);

    let tks = quote! {
        impl<T> TupleSame<T> for (#(#ts),*) { }

        impl<#(#nts),*> Tuple for (#(#nts),*) {
            fn size(&self) -> usize {
                #size_lit
            }
        }

        impl<'a, #(#ants),*> TupleAsRef<'a> for (#(#nts),*) {
            type OutTuple = (#(#ref_nts),*);

            #[doc = #ref_doc]
            fn as_ref(&'a self) -> Self::OutTuple {
                (#(#ref_impl),*)
            }
        }

        impl<'a, #(#ants),*> TupleAsMut<'a> for (#(#nts),*) {
            type OutTuple = (#(#mut_nts),*);

            #[doc = #mut_doc]
            fn as_mut(&'a mut self) -> Self::OutTuple {
                (#(#mut_impl),*)
            }
        }
    };
    tks
}

fn gen_tuple_n_impl(ctx: &Ctx, out_dir: &OsString) {
    let item_names = (0..34usize).into_iter().map(|i| format_ident!("Item{}", i)).collect::<Vec<_>>();
    let type_items = item_names.iter().map(|i| quote! { type #i; }).collect::<Vec<_>>();
    let let_items = item_names.iter().zip(ctx.nts.iter()).map(|(i, t)| quote! { type #i = #t; }).collect::<Vec<_>>();
    let items = (2..33usize)
        .into_iter()
        .map(|i| gen_tuple_n_impl_size(ctx, i, &type_items, &let_items));
    let tks = quote! { #(#items)* };
    let code = tks.to_string();
    let dest_path = Path::new(out_dir).join("tuple_n.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_tuple_n_impl_size(ctx: &Ctx, size: usize, type_items: &[TokenStream], let_items: &[TokenStream]) -> TokenStream {
    let tuple_name = format_ident!("Tuple{}", size);

    let nts = &ctx.nts[0..size];

    let type_items = &type_items[0..size];
    let let_items = &let_items[0..size];

    let tks = quote! {
        pub trait #tuple_name: Tuple {
            #(#type_items)*
        }
        impl<#(#nts),*> #tuple_name for (#(#nts),*) {
            #(#let_items)*
        }
    };
    tks
}

#[cfg(feature = "tuple_iter")]
fn gen_tuple_iter(ctx: &Ctx, out_dir: &OsString) {
    let items = (2..33usize)
        .into_iter()
        .map(|i| gen_tuple_iter_size(ctx, i));
    let tks = quote! { #(#items)* };
    let code = tks.to_string();
    let dest_path = Path::new(out_dir).join("tuple_iter.rs");
    fs::write(&dest_path, code).unwrap();
}

#[cfg(feature = "tuple_iter")]
fn gen_tuple_iter_size(ctx: &Ctx, size: usize) -> TokenStream {
    let size_lit = &ctx.size_lits[size];
    let last_lit = &ctx.size_lits[size - 1];

    let iter_struct_name = format_ident!("Tuple{}Iter", size);
    let into_iter_struct_name = format_ident!("Tuple{}IntoIter", size);

    let ts = &ctx.ts[0..size];
    let ut = quote! { MaybeUninit<T> };
    let uts = (0..size).into_iter().map(|_| &ut);
    let utts = ctx.size_lits[0..size]
        .iter()
        .map(|i| quote! { MaybeUninit::new(t.#i) });
    let iter_match = ctx.size_lits[0..size]
        .iter()
        .map(|i| quote! { #i => &self.1 .#i, })
        .collect::<Vec<_>>();
    let into_match = ctx.size_lits[0..size]
        .iter()
        .zip(ctx.size_lits[1..size + 1].iter())
        .map(|(i, n)| quote! { #i => std::mem::replace(&mut self.#n, MaybeUninit::uninit()), })
        .collect::<Vec<_>>();
    let from = quote! { iter.next().unwrap() };
    let froms = (0..size).into_iter().map(|_| &from);

    let derive_iter = if size > 12 {
        quote! {}
    } else {
        quote! {#[derive(Debug, Clone, Copy)]}
    };
    let derive_into = if size > 12 {
        quote! {}
    } else {
        quote! {#[derive(Debug)]}
    };

    let tks = quote! {
        #derive_iter
        #[doc(hidden)]
        pub struct #iter_struct_name<'a, T>(usize, &'a (#(#ts),*));
        impl<'a, T> #iter_struct_name<'a, T> {
            #[inline]
            pub fn new(t: &'a (#(#ts),*)) -> Self {
                Self(0, t)
            }
        }
        #derive_into
        #[doc(hidden)]
        pub struct #into_iter_struct_name<T>(usize, #(#uts),*);
        impl<T> #into_iter_struct_name<T> {
            #[inline]
            pub fn new(t: (#(#ts),*)) -> Self {
                Self(0, #(#utts),*)
            }
        }

        impl<'a, T> Iterator for #iter_struct_name<'a, T> {
            type Item = &'a T;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let res = match self.0 {
                    #(#iter_match)*
                    _ => return None
                };
                self.0 += 1;
                Some(res)
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let exact = self.len();
                (exact, Some(exact))
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                if n < self.0 { return None }
                let res = match self.0 {
                    #(#iter_match)*
                    _ => return None
                };
                self.0 = min(n + 1, #size_lit);
                Some(res)
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                if self.len() == 0 { return None }
                Some(&self.1 .#last_lit)
            }
        }
        impl<'a, T> ExactSizeIterator for #iter_struct_name<'a, T> {
            #[inline]
            fn len(&self) -> usize { #size_lit - self.0 }
        }
        impl<'a, T> FusedIterator for #iter_struct_name<'a, T> { }
        impl<'a, T> AsRef<(#(#ts),*)> for #iter_struct_name<'a, T> {
            #[inline]
            fn as_ref(&self) -> &(#(#ts),*) {
                self.1
            }
        }
        impl<'a, T: 'a> TupleIter<'a> for (#(#ts),*) {
            type Iter = #iter_struct_name<'a, T>;

            #[inline]
            fn iter(&'a self) -> Self::Iter {
                #iter_struct_name::new(self)
            }
        }

        impl<T> Iterator for #into_iter_struct_name<T> {
            type Item = T;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                let res = match self.0 {
                    #(#into_match)*
                    _ => return None
                };
                self.0 += 1;
                Some(unsafe { res.assume_init() })
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let exact = self.len();
                (exact, Some(exact))
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }

            #[inline]
            fn nth(&mut self, n: usize) -> Option<Self::Item> {
                if n < self.0 { return None }
                let res = match self.0 {
                    #(#into_match)*
                    _ => return None
                };
                self.0 = min(n + 1, #size_lit);
                Some(unsafe { res.assume_init() })
            }

            #[inline]
            fn last(self) -> Option<Self::Item> {
                if self.len() == 0 { return None }
                Some(unsafe { self.#size_lit.assume_init() })
            }
        }
        impl<T> ExactSizeIterator for #into_iter_struct_name<T> {
            #[inline]
            fn len(&self) -> usize { #size_lit - self.0 }
        }
        impl<T> FusedIterator for #into_iter_struct_name<T> { }
        impl<T> TupleIntoIter for (#(#ts),*) {
            type Iter = #into_iter_struct_name<T>;

            #[inline]
            fn into_iter(self) -> Self::Iter {
                #into_iter_struct_name::new(self)
            }
        }

        impl<T> TupleFromIter<T> for (#(#ts),*) {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut iter = iter.into_iter();
                (#(#froms),*)
            }
        }
    };
    tks
}

#[cfg(feature = "tuple_map")]
fn gen_tuple_map(ctx: &Ctx, out_dir: &OsString) {
    let items = (2..33usize).into_iter().map(|i| gen_tuple_map_size(ctx, i));
    let tks = quote! { #(#items)* };
    let code = tks.to_string();
    let dest_path = Path::new(out_dir).join("tuple_map.rs");
    fs::write(&dest_path, code).unwrap();
}

#[cfg(feature = "tuple_map")]
fn gen_tuple_map_size(ctx: &Ctx, size: usize) -> TokenStream {
    let items = if size > 16 {
        vec![]
    } else {
        (0..size)
            .into_iter()
            .map(|n| gen_tuple_map_n_size(ctx, size, n))
            .collect()
    };

    let map_name = format_ident!("Tuple{}Map", size);

    let ts = &ctx.ts[0..size];
    let us = &ctx.us[0..size];

    let map_impl = ctx.size_lits[0..size].iter().map(|l| {
        quote! {
            f(self.#l)
        }
    });

    let map_doc = format!("Mapping for Tuple{}", size);

    let tks = quote! {
        #(#items)*

        #[doc = #map_doc]
        pub trait #map_name<T> {
            #[doc = #map_doc]
            fn map<U>(self, f: impl FnMut(T) -> U) -> (#(#us),*);
        }
        impl<T> #map_name<T> for (#(#ts),*) {
            fn map<U>(self, mut f: impl FnMut(T) -> U) -> (#(#us),*) {
                (#(#map_impl),*)
            }
        }
    };
    tks
}

#[cfg(feature = "tuple_map")]
fn gen_tuple_map_n_size(ctx: &Ctx, size: usize, n: usize) -> TokenStream {
    let t = &ctx.nts[n];
    let map_n_name = format_ident!("Tuple{}Map{}", size, n);
    let map_n = format_ident!("map{}", n);

    let rts = &ctx.nts[0..size];
    let ts = ctx.nts[0..size]
        .iter()
        .enumerate()
        .map(|(i, l)| if i == n { ctx.u } else { l })
        .collect::<Vec<_>>();

    let impls = ctx.size_lits[0..size].iter().enumerate().map(|(i, l)| {
        if i == n {
            quote! { f(self.#l) }
        } else {
            quote! { self.#l }
        }
    });

    let doc = format!("Mapping `.{}` for Tuple{}", n, size);

    let tks = quote! {
        #[doc=#doc]
        pub trait #map_n_name<#(#rts),*> {
            #[doc=#doc]
            fn #map_n<U>(self, f: impl FnOnce(#t) -> U) -> (#(#ts),*);
        }
        impl<#(#rts),*> #map_n_name<#(#rts),*> for (#(#rts),*) {
            fn #map_n<U>(self, f: impl FnOnce(#t) -> U) -> (#(#ts),*) {
                (#(#impls),*)
            }
        }
    };
    tks
}
