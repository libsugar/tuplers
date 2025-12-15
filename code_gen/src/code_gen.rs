use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use std::{fs, path::Path};
use syn::LitInt;

const AUTO_GEN_TIP: &'static str = "// auto generated code, do not modify\n\n";

macro_rules! tif {
    { $c:expr => $t:expr ; $e:expr } => { if $c { $t } else { $e } };
}

pub fn code_gen(out_dir: &Path) {
    let t = format_ident!("T");
    let u = format_ident!("U");

    let ctx = init(33, &t, &u);

    gen_tuple_impl(&ctx, &out_dir);
    gen_tuple_item_n_impl(&ctx, &out_dir);
    gen_tuple_alias_macro(&ctx, &out_dir);
    gen_get_dyn(&ctx, &out_dir);
    gen_get_const(&ctx, &out_dir);
    gen_clone(&ctx, &out_dir);
    gen_mapper(&ctx, &out_dir);
    gen_convert(&ctx, &out_dir);

    gen_tuple_iter(&ctx, &out_dir);
    gen_combin(&ctx, &out_dir);
    gen_split_parts(&ctx, &out_dir);
    gen_split_by(&ctx, &out_dir);
    gen_split_to_tuple_by(&ctx, &out_dir);
    gen_split_at(&ctx, &out_dir);
    gen_split_to_tuple_at(&ctx, &out_dir);
    gen_transpose(&ctx, &out_dir);
    gen_flatten(&ctx, &out_dir);
    gen_call(&ctx, &out_dir);
    gen_apply_tuple(&ctx, &out_dir);
    gen_capt(&ctx, &out_dir);
    gen_permutations(&ctx, &out_dir);
    gen_combinations(&ctx, &out_dir);
    gen_uniform_map(&ctx, &out_dir);
    gen_uniform_map_by(&ctx, &out_dir);
}

#[allow(dead_code)]
struct Ctx<'a> {
    pub t: &'a Ident,
    pub u: &'a Ident,
    pub size_lits: Vec<LitInt>,
    pub ts: Vec<&'a Ident>,
    pub us: Vec<&'a Ident>,
    pub nts: Vec<Ident>,
    pub nus: Vec<Ident>,
    pub nms: Vec<Ident>,
    pub nvs: Vec<Ident>,
    pub nvms: Vec<Ident>,
    pub nfs: Vec<Ident>,
    pub ntfs: Vec<Ident>,
    pub ants: Vec<TokenStream>,
}

fn init<'a>(max: usize, t: &'a Ident, u: &'a Ident) -> Ctx<'a> {
    let size_lits = (0..max + 1).into_iter().map(|i| LitInt::new(i.to_string().as_str(), Span::call_site())).collect::<Vec<_>>();
    let ts = (0..max + 1).into_iter().map(|_| t).collect::<Vec<_>>();
    let us = (0..max + 1).into_iter().map(|_| u).collect::<Vec<_>>();
    let nts = (0..max + 1).into_iter().map(|i| format_ident!("T{}", i)).collect::<Vec<_>>();
    let nus = (0..max + 1).into_iter().map(|i| format_ident!("U{}", i)).collect::<Vec<_>>();
    let nms = (0..max + 1).into_iter().map(|i| format_ident!("M{}", i)).collect::<Vec<_>>();
    let nvs = (0..max + 1).into_iter().map(|i| format_ident!("v{}", i)).collect::<Vec<_>>();
    let nvms = (0..max + 1).into_iter().map(|i| format_ident!("m{}", i)).collect::<Vec<_>>();
    let nfs = (0..max + 1).into_iter().map(|i| format_ident!("f{}", i)).collect::<Vec<_>>();
    let ntfs = (0..max + 1).into_iter().map(|i| format_ident!("F{}", i)).collect::<Vec<_>>();
    let ants = nts[0..max + 1].iter().map(|i| quote! { #i: 'a }).collect::<Vec<_>>();
    let ctx = Ctx { t, u, size_lits, ts, us, nts, nus, nms, nvs, nvms, nfs, ntfs, ants };
    ctx
}

fn gen_tuple_impl(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_tuple_impl_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("tuple_impl.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_tuple_impl_size(ctx: &Ctx, size: usize) -> TokenStream {
    let size_lit = &ctx.size_lits[size];
    let ts = &ctx.ts[0..size];
    let nts = &ctx.nts[0..size];

    let tks = quote! {
        impl<T> AnyHomogeneousTuple<T> for (#(#ts),*) { }
        impl<T> HomogeneousTuple<T> for (#(#ts),*) { }

        impl<#(#nts),*> AnyTuple for (#(#nts),*) {
            fn arity(&self) -> usize {
                #size_lit
            }
        }

        impl<#(#nts),*> Tuple for (#(#nts),*) {
            const ARITY: usize = #size_lit;

            type Item<const N: usize>
                = <Self as crate::TupleItem<N>>::ItemN
            where
                Self: crate::TupleItem<N>;
        }
    };
    tks
}

fn gen_tuple_item_n_impl(ctx: &Ctx, out_dir: &Path) {
    let items = (1..33usize).into_iter().map(|i| gen_tuple_item_n_impl_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("tuple_item_n.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_tuple_item_n_impl_size(ctx: &Ctx, size: usize) -> TokenStream {
    let tks = (0..size).into_iter().map(|n| gen_tuple_item_n_impl_size_n(ctx, size, n));
    quote! { #(#tks)* }
}

fn gen_tuple_item_n_impl_size_n(ctx: &Ctx, size: usize, n: usize) -> TokenStream {
    let n_lit = &ctx.size_lits[n];
    let nts = &ctx.nts[0..size];
    let nt = &ctx.nts[n];

    let tks = quote! {
        impl<#(#nts),*> TupleNthItem<#n_lit, #nt> for (#(#nts),*,) { }

        impl<#(#nts),*> TupleItem<#n_lit> for (#(#nts),*,) {
            type ItemN = #nt;
        }
    };
    tks
}

fn gen_mapper(ctx: &Ctx, out_dir: &Path) {
    let items = (1..33usize).map(|i| gen_mapper_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("map.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_mapper_size(ctx: &Ctx, size: usize) -> TokenStream {
    let tuple_n = (0..size).map(|n| gen_mapper_n_size(ctx, size, n));

    let map_all = if size > 1 {
        let nts = &ctx.nts[0..size];
        let nms = &ctx.nms[0..size];
        let nvms = &ctx.nvms[0..size];
        let lits = &ctx.size_lits[0..size];

        quote! {
            impl<#(#nts,)* M> TupleMapAll<M> for (#(#nts,)*)
            where
               M: TupleMapperMut<Self> #(+ TupleMapperMutN<#lits, Self>)*,
            {
               type Output = (#(M::Output<#lits>,)*);

               fn map_all(self, mut mapper: M) -> Self::Output {
                   (#(mapper.do_map_mut(self.#lits),)*)
               }
            }

            impl<#(#nts,)* #(#nms,)*> TupleMapAll<(#(#nms,)*)> for (#(#nts,)*)
            where
                #(#nms: TupleMapperOnce<Self> + TupleMapperOnceN<#lits, Self>,)*
            {
                type Output = (#(#nms::Output<#lits>,)*);

                fn map_all(self, mapper: (#(#nms,)*)) -> Self::Output {
                    let (#(#nvms,)*) = mapper;
                    (#(#nvms.do_map_once(self.#lits),)*)
                }
            }
        }
    } else {
        quote! {}
    };

    let ts = &ctx.ts[0..size];
    let lits = &ctx.size_lits[0..size];
    let vars: Vec<_> = lits.iter().map(|lit| format_ident!("v{lit}")).collect();

    let tks = quote! {
        #(#tuple_n)*

        impl<T> TupleDynamicMap<T> for (#(#ts,)*) {
            fn dyn_map(self, n: usize, mapper: impl FnOnce(T) -> T) -> Result<Self, Self> {
                let (#(mut #vars,)*) = self;
                match n {
                    #(#lits => #vars = (mapper)(#vars),)*
                    _ => return Err((#(#vars,)*)),
                }
                Ok((#(#vars,)*))
            }
        }

        #map_all
    };
    tks
}

fn gen_mapper_n_size(ctx: &Ctx, size: usize, n: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let lit = &ctx.size_lits[n];

    let outputs = (0..size).into_iter().map(|i| {
        let lit = &ctx.size_lits[i];
        let nt = &ctx.nts[i];
        if i == n {
            quote! { M::Output<#lit> }
        } else {
            quote! { #nt }
        }
    });

    let mapping = (0..size).into_iter().map(|i| {
        let lit = &ctx.size_lits[i];
        if i == n {
            quote! { mapper.do_map_once(self.#lit) }
        } else {
            quote! { self.#lit }
        }
    });

    let tks = quote! {
        impl<#(#nts,)* M> TupleMapN<#lit, M> for (#(#nts,)*)
        where
            M: TupleMapperMut<Self> + TupleMapperOnceN<#lit, Self>,
        {
            type OutputN = (#(#outputs,)*);

            fn map_n(self, mapper: M) -> Self::OutputN {
                (#(#mapping,)*)
            }
        }
    };
    tks
}

fn gen_convert(ctx: &Ctx, out_dir: &Path) {
    let items = (0..33usize).map(|i| gen_convert_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("convert.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_convert_size(ctx: &Ctx, size: usize) -> TokenStream {
    let size_lits = &ctx.size_lits[0..size];
    let nts = &ctx.nts[0..size];
    let nus = &ctx.nus[0..size];

    let tks = quote! {
        impl<'a, #(#nts: 'a,)*> TupleAsRef<'a> for (#(#nts,)*) {
            type Output = (#(&'a #nts,)*);

            fn as_ref(&'a self) -> Self::Output {
                (#(&self.#size_lits,)*)
            }
        }

        impl<'a, #(#nts: 'a,)*> TupleAsMut<'a> for (#(#nts,)*) {
            type Output = (#(&'a mut #nts,)*);

            fn as_mut(&'a mut self) -> Self::Output {
                (#(&mut self.#size_lits,)*)
            }
        }

        impl<'a, #(#nts: 'a + Deref,)*> TupleAsDeref<'a> for (#(#nts,)*) {
            type Output = (#(&'a <#nts as Deref>::Target,)*);

            fn as_deref(&'a self) -> Self::Output {
                (#(self.#size_lits.deref(),)*)
            }
        }

        impl<'a, #(#nts: 'a + DerefMut,)*> TupleAsDerefMut<'a> for (#(#nts,)*) {
            type Output = (#(&'a mut <#nts as Deref>::Target,)*);

            fn as_deref_mut(&'a mut self) -> Self::Output {
                (#(self.#size_lits.deref_mut(),)*)
            }
        }

        impl<#(#nts,)*> TupleAsOption for (#(#nts,)*) {
            type Output = (#(Option<#nts>,)*);

            fn as_some(self) -> Self::Output {
                (#(Some(self.#size_lits),)*)
            }
        }

        impl<E, #(#nts,)*> TupleAsResultOk<E> for (#(#nts,)*) {
            type Output = (#(Result<#nts, E>,)*);

            fn as_ok(self) -> Self::Output {
                (#(Ok(self.#size_lits),)*)
            }
        }

        impl<O, #(#nts,)*> TupleAsResultErr<O> for (#(#nts,)*) {
            type Output = (#(Result<O, #nts>,)*);

            fn as_err(self) -> Self::Output {
                (#(Err(self.#size_lits),)*)
            }
        }

        impl<U, #(#nts),* > AnyTupleAllInto<U> for (#(#nts,)*) where #(#nts: Into<U>,)* { }
        impl<U, #(#nts),* > AnyTupleAllFrom<U> for (#(#nts,)*) where #(#nts: From<U>,)* { }

        impl<U, #(#nts),* > TupleAllInto<U> for (#(#nts,)*) where #(#nts: Into<U>,)* {
            type Item<const N: usize> = <Self as TupleItem<N>>::ItemN
            where
                Self: TupleItem<N>,
                <Self as TupleItem<N>>::ItemN: Into<U>;
        }

        impl<U, #(#nts),* > TupleAllFrom<U> for (#(#nts,)*) where #(#nts: From<U>,)* {
            type Item<const N: usize> = <Self as TupleItem<N>>::ItemN
            where
                Self: TupleItem<N>,
                <Self as TupleItem<N>>::ItemN: From<U>;
        }

        impl<#(#nts,)* #(#nus,)* > TupleInto<(#(#nus,)*)> for (#(#nts,)*)
        where #(#nts: Into<#nus>,)* {
            fn tuple_into(self) -> (#(#nus,)*)
            {
                (#(self.#size_lits.into(),)*)
            }
        }
        impl<#(#nts,)* #(#nus,)* > TupleFrom<(#(#nus,)*)> for (#(#nts,)*)
        where #(#nts: From<#nus>,)* {
            fn tuple_from(src: (#(#nus,)*)) -> Self
            {
                (#(src.#size_lits.into(),)*)
            }
        }
        impl<#(#nts,)* #(#nus,)* > TupleTryInto<(#(#nus,)*)> for (#(#nts,)*)
        where #(#nts: TryInto<#nus>,)* {
            type Output = (#(Result<#nus, #nts::Error>,)*);

            fn tuple_try_into(self) -> Self::Output
            {
                (#(self.#size_lits.try_into(),)*)
            }
        }
        impl<#(#nts,)* #(#nus,)* > TupleTryFrom<(#(#nus,)*)> for (#(#nts,)*)
        where #(#nts: TryFrom<#nus>,)* {
            type Output = (#(Result<#nts, #nts::Error>,)*);

            fn tuple_try_from(src: (#(#nus,)*)) -> Self::Output
            {
                (#(src.#size_lits.try_into(),)*)
            }
        }
    };
    tks
}

fn gen_tuple_alias_macro(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_tuple_alias_macro_size(ctx, i));
    let tks = quote! {
        #[doc(hidden)]
        #[macro_export(local_inner_macros)]
        macro_rules! tuple_ {
            { $t:ty ; 0 } => { () };
            { $t:expr ; 0 } => { () };
            { $t:ty ; 1 } => { ($t,) };
            { $t:expr ; 1 } => { ($t,) };
            #(#items)*
        }
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("tuple_alias.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_tuple_alias_macro_size(ctx: &Ctx, size: usize) -> TokenStream {
    let size_lit = &ctx.size_lits[size];

    let ty = quote! { $t };
    let tys = (0..size).into_iter().map(|_| &ty).collect::<Vec<_>>();

    let ntys = (0..size + 1).map(|i| format_ident!("t{}", i)).map(|i| quote! { $#i }).collect::<Vec<_>>();

    let items = (0..size + 1).map(|i| gen_tuple_alias_macro_size_n(ctx, size, i, &ntys));

    let tks = quote! {
        { $t:ty ; #size_lit } => { (#(#tys),*) };
        { $t:expr ; #size_lit } => { (#(#tys),*) };
        #(#items)*
    };
    tks
}

fn gen_tuple_alias_macro_size_n(ctx: &Ctx, size: usize, n: usize, ntys: &[TokenStream]) -> TokenStream {
    let size_lit = &ctx.size_lits[size];

    let u = quote! { _ };
    let nntys = &ntys[0..n];
    let tys = ntys[0..size].iter().enumerate().map(|(i, l)| if i < n { l } else { &u });

    let tks = quote! {
        { #size_lit; #(#nntys:ty),* } => { (#(#tys),*) };
    };
    tks
}

fn gen_tuple_iter(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_tuple_iter_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("tuple_iter.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_tuple_iter_size(ctx: &Ctx, size: usize) -> TokenStream {
    let size_lit = &ctx.size_lits[size];
    let size_lits = &ctx.size_lits[0..size];

    let iter_struct_name = format_ident!("Tuple{}Iter", size);
    let into_iter_struct_name = format_ident!("Tuple{}IntoIter", size);

    let ts = &ctx.ts[0..size];

    let from = quote! { iter.next() };
    let froms = (0..size).into_iter().map(|_| &from).collect::<Vec<_>>();

    let derive_iter = tif! {size > 12 =>  quote! {} ; quote! {#[derive(Debug, Clone)]} };
    let derive_into = tif! {size > 12 =>  quote! {} ; quote! {#[derive(Debug)]} };

    let iter_ = quote! {
        #derive_iter
        pub struct #iter_struct_name<'a, T>([&'a T; #size_lit], Range<usize>);
        impl<'a, T> #iter_struct_name<'a, T> {
            #[inline]
            pub fn new(t: &'a (#(#ts),*)) -> Self {
                Self([#(&t.#size_lits),*], 0..#size_lit)
            }
        }

        impl<'a, T> Iterator for #iter_struct_name<'a, T> {
            type Item = &'a T;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.1.next().map(|idx| unsafe { *self.0.get_unchecked(idx) })
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len();
                (len, Some(len))
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }

            #[inline]
            fn last(mut self) -> Option<Self::Item> {
                self.next_back()
            }
        }
        impl<'a, T> DoubleEndedIterator for #iter_struct_name<'a, T> {
            #[inline]
            fn next_back(&mut self) -> Option<Self::Item> {
                self.1.next_back().map(|idx| unsafe { *self.0.get_unchecked(idx) })
            }
        }
        impl<'a, T> ExactSizeIterator for #iter_struct_name<'a, T> {
            #[inline]
            fn len(&self) -> usize { self.1.end - self.1.start }
        }
        impl<'a, T> FusedIterator for #iter_struct_name<'a, T> { }
        impl<'a, T: 'a> TupleIter<'a> for (#(#ts),*) {
            type Iter = #iter_struct_name<'a, T>;

            #[inline]
            fn iter(&'a self) -> Self::Iter {
                #iter_struct_name::new(self)
            }
        }
    };

    let into_ = quote! {
        #derive_into
        pub struct #into_iter_struct_name<T>([MaybeUninit<T>; #size_lit], Range<usize>);
        impl<T> #into_iter_struct_name<T> {
            #[inline]
            pub fn new(t: (#(#ts),*)) -> Self {
                Self([#(MaybeUninit::new(t.#size_lits)),*], 0..#size_lit)
            }
        }
        impl<T> Iterator for #into_iter_struct_name<T> {
            type Item = T;

            #[inline]
            fn next(&mut self) -> Option<Self::Item> {
                self.1.next().map(|idx| unsafe {
                    core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init()
                })
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                let len = self.len();
                (len, Some(len))
            }

            #[inline]
            fn count(self) -> usize {
                self.len()
            }

            #[inline]
            fn last(mut self) -> Option<Self::Item> {
                self.next_back()
            }
        }
        impl<T> DoubleEndedIterator for #into_iter_struct_name<T> {
            fn next_back(&mut self) -> Option<Self::Item> {
                self.1.next_back().map(|idx| unsafe {
                    core::mem::replace(self.0.get_unchecked_mut(idx), MaybeUninit::uninit()).assume_init()
                })
            }
        }
        impl<T> ExactSizeIterator for #into_iter_struct_name<T> {
            #[inline]
            fn len(&self) -> usize { self.1.end - self.1.start }
        }
        impl<T> FusedIterator for #into_iter_struct_name<T> { }
        impl<T> TupleIntoIter for (#(#ts),*) {
            type Iter = #into_iter_struct_name<T>;

            #[inline]
            fn into_iter(self) -> Self::Iter {
                #into_iter_struct_name::new(self)
            }
        }
        impl<T> Drop for #into_iter_struct_name<T> {
            fn drop(&mut self) {
                let slice = unsafe { self.0.get_unchecked_mut(self.1.clone()) };
                let slice = unsafe { &mut *(slice as *mut [MaybeUninit<T>] as *mut [T]) };
                unsafe { core::ptr::drop_in_place(slice) }
            }
        }
    };

    let tks = quote! {
        #iter_
        #into_

        impl<T> TupleFromIter<T> for (#(#ts),*) {
            fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
                let mut iter = iter.into_iter();
                (#(#froms.unwrap()),*)
            }
        }

        impl<T> TupleTryFromIter<T> for (#(#ts),*) {
            fn try_from_iter<I: IntoIterator<Item = T>>(iter: I) -> Option<Self> {
                let mut iter = iter.into_iter();
                Some((#(#froms?),*))
            }
        }

        impl<T> TupleFromIterTry<T> for (#(#ts),*) {
            type OutTuple = (#(Option<#ts>),*);

            fn from_iter_try<I: IntoIterator<Item = T>>(iter: I) -> Self::OutTuple {
                let mut iter = iter.into_iter();
                (#(#froms),*)
            }
        }
    };
    tks
}

fn gen_combin(ctx: &Ctx, out_dir: &Path) {
    let self_impl = ctx.size_lits.iter().map(|i| quote! { self.#i }).collect::<Vec<_>>();
    let target_impl = ctx.size_lits.iter().map(|i| quote! { target.#i }).collect::<Vec<_>>();
    let items = (2..33usize).map(|i| gen_combin_size(ctx, i));
    let concats = (0..17usize).flat_map(|a| {
        let self_impl = &self_impl;
        let target_impl = &target_impl;
        (0..17usize).map(move |b| gen_combin_concat_size(ctx, a, b, &self_impl, &target_impl))
    });
    let tks = quote! {
        #(#items)*
        #(#concats)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("combin.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_combin_size(ctx: &Ctx, size: usize) -> TokenStream {
    let ts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let tks = quote! {
        impl<T, #(#ts),*> CombinLeft<T> for (#(#ts),*) {
            type Out = (T, #(#ts),*);

            fn push_left(self, target: T) -> Self::Out {
                (target, #(self.#size_lits),*)
            }
        }
        impl<T, #(#ts),*> CombinRight<T> for ( #(#ts),*) {
            type Out = ( #(#ts),*, T);

            fn push_right(self, target: T) -> Self::Out {
                (#(self.#size_lits),*, target)
            }
        }
    };
    tks
}

fn gen_combin_concat_size(ctx: &Ctx, sizea: usize, sizeb: usize, self_impl: &[TokenStream], target_impl: &[TokenStream]) -> TokenStream {
    let ants = &ctx.nts[0..sizea];
    let bnts = &ctx.nts[sizea..sizea + sizeb];
    let gnts = &ctx.nts[0..sizea + sizeb];
    let atc = tif! { ants.len() == 1 => quote! { , } ; quote! { } };
    let btc = tif! { bnts.len() == 1 => quote! { , } ; quote! { } };
    let gtc = tif! { gnts.len() == 1 => quote! { , } ; quote! { } };
    let impls = self_impl[0..sizea].iter().chain(target_impl[0..sizeb].iter());
    let tks = quote! {
        impl<#(#gnts),*> CombinConcat<(#(#bnts),*#btc)> for (#(#ants),*#atc) {
            type Out = (#(#gnts),*#gtc);

            #[allow(unused_variables)]
            fn concat(self, target: (#(#bnts),*#btc)) -> Self::Out {
                (#(#impls),*#gtc)
            }
        }
    };
    tks
}

fn gen_split_parts(ctx: &Ctx, out_dir: &Path) {
    let items = (2..16usize).map(|i| gen_split_parts_n(ctx, i));
    let tks = quote! {
        #(#items)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("split_parts.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_split_parts_n(ctx: &Ctx, n: usize) -> TokenStream {
    let trait_name = format_ident!("TupleSplit{}Parts", n);
    let fn_name = format_ident!("split_{}_parts", n);
    let trait_doc = format!("Split into {} parts", n);

    let impls = (n * 2..33usize).map(|i| gen_split_parts_n_impl_size(ctx, &trait_name, &fn_name, n, i));

    let tks = quote! {
        #[doc = #trait_doc]
        pub trait #trait_name {
            type OutTuple;

            #[doc = #trait_doc]
            fn #fn_name(self) -> Self::OutTuple;
        }

        #(#impls)*
    };
    tks
}

fn gen_split_parts_n_impl_size(ctx: &Ctx, trait_name: &Ident, fn_name: &Ident, n: usize, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let d = size / n;
    let m = size % n;

    let out_type = (0..n)
        .map(|i| {
            let r = i * d + std::cmp::min(i, m)..(i + 1) * d + std::cmp::min(i + 1, m);
            let nts = &nts[r];
            quote! { (#(#nts),*) }
        })
        .collect::<Vec<_>>();

    let out_lit = (0..n)
        .map(|i| {
            let r = i * d + std::cmp::min(i, m)..(i + 1) * d + std::cmp::min(i + 1, m);
            let size_lits = &size_lits[r];
            quote! { (#(self.#size_lits),*) }
        })
        .collect::<Vec<_>>();

    let tks = quote! {
        impl<#(#nts),*> #trait_name for (#(#nts),*) {
            type OutTuple = (#(#out_type),*);

            fn #fn_name(self) -> Self::OutTuple {
                (#(#out_lit),*)
            }
        }
    };
    tks
}

fn gen_split_by(ctx: &Ctx, out_dir: &Path) {
    let items = (2..32usize).map(|i| gen_split_by_n(ctx, i));
    let tks = quote! {
        #(#items)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("split_by.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_split_by_n(ctx: &Ctx, n: usize) -> TokenStream {
    let trait_name = format_ident!("TupleSplitBy{}", n);
    let fn_name = format_ident!("split_by_{}", n);
    let trait_doc = format!("Split by {}", n);

    let impls = (n..33usize).map(|i| gen_split_by_n_impl_size(ctx, &trait_name, &fn_name, n, i));

    let tks = quote! {
        #[doc = #trait_doc]
        pub trait #trait_name {
            type OutTuple;

            #[doc = #trait_doc]
            fn #fn_name(self) -> Self::OutTuple;
        }

        #(#impls)*
    };
    tks
}

fn gen_split_by_n_impl_size(ctx: &Ctx, trait_name: &Ident, fn_name: &Ident, n: usize, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let items_range = 0..size / n + if size % n == 0 { 0 } else { 1 };

    let out_type = items_range
        .clone()
        .map(|i| {
            let start = i * n;
            let nts = &nts[start..std::cmp::min(start + n, size)];
            if nts.len() == 1 {
                quote! { #(#nts),* }
            } else {
                quote! { (#(#nts),*) }
            }
        })
        .collect::<Vec<_>>();
    let out_type = if out_type.len() == 1 {
        quote! { #(#out_type),* }
    } else {
        quote! { (#(#out_type),*) }
    };

    let out_lit = items_range
        .map(|i| {
            let start = i * n;
            let size_lits = &size_lits[start..std::cmp::min(start + n, size)];
            if size_lits.len() == 1 {
                quote! { #(self.#size_lits),* }
            } else {
                quote! { (#(self.#size_lits),*) }
            }
        })
        .collect::<Vec<_>>();
    let out_lit = if out_lit.len() == 1 {
        quote! { #(#out_lit),* }
    } else {
        quote! { (#(#out_lit),*) }
    };

    let tks = quote! {
        impl<#(#nts),*> #trait_name for (#(#nts),*) {
            type OutTuple = #out_type;

            fn #fn_name(self) -> Self::OutTuple {
                #out_lit
            }
        }
    };
    tks
}

fn gen_split_to_tuple_by(ctx: &Ctx, out_dir: &Path) {
    let items = (2..32usize).map(|i| gen_split_to_tuple_by_n(ctx, i));
    let tks = quote! {
        #(#items)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("split_to_tuple_by.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_split_to_tuple_by_n(ctx: &Ctx, n: usize) -> TokenStream {
    let trait_name = format_ident!("TupleSplitToTupleBy{}", n);
    let fn_name = format_ident!("split_to_tuple_by_{}", n);
    let trait_doc = format!("Split to tuple by {}", n);

    let impls = (n..33usize).map(|i| gen_split_to_tuple_by_n_impl_size(ctx, &trait_name, &fn_name, n, i));

    let tks = quote! {
        #[doc = #trait_doc]
        pub trait #trait_name {
            type OutTuple;

            #[doc = #trait_doc]
            fn #fn_name(self) -> Self::OutTuple;
        }

        #(#impls)*
    };
    tks
}

fn gen_split_to_tuple_by_n_impl_size(ctx: &Ctx, trait_name: &Ident, fn_name: &Ident, n: usize, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let items_range = 0..size / n + if size % n == 0 { 0 } else { 1 };

    let out_type = items_range
        .clone()
        .map(|i| {
            let start = i * n;
            let nts = &nts[start..std::cmp::min(start + n, size)];
            if nts.len() == 1 {
                quote! { (#(#nts),*,) }
            } else {
                quote! { (#(#nts),*) }
            }
        })
        .collect::<Vec<_>>();
    let out_type = if out_type.len() == 1 {
        quote! { (#(#out_type),*,) }
    } else {
        quote! { (#(#out_type),*) }
    };

    let out_lit = items_range
        .map(|i| {
            let start = i * n;
            let size_lits = &size_lits[start..std::cmp::min(start + n, size)];
            if size_lits.len() == 1 {
                quote! { (#(self.#size_lits),*,) }
            } else {
                quote! { (#(self.#size_lits),*) }
            }
        })
        .collect::<Vec<_>>();
    let out_lit = if out_lit.len() == 1 {
        quote! { (#(#out_lit),*,) }
    } else {
        quote! { (#(#out_lit),*) }
    };

    let tks = quote! {
        impl<#(#nts),*> #trait_name for (#(#nts),*) {
            type OutTuple = #out_type;

            fn #fn_name(self) -> Self::OutTuple {
                #out_lit
            }
        }
    };
    tks
}

fn gen_split_at(ctx: &Ctx, out_dir: &Path) {
    let items = (1..32usize).map(|i| gen_split_at_n(ctx, i));
    let tks = quote! {
        #(#items)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("split_at.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_split_at_n(ctx: &Ctx, n: usize) -> TokenStream {
    let trait_name = format_ident!("TupleSplitAt{}", n);
    let fn_name = format_ident!("split_at_{}", n);
    let trait_doc = format!("Split at {}", n);

    let impls = (n..33usize).map(|i| gen_split_at_n_impl_size(ctx, &trait_name, &fn_name, n, i));

    let tks = quote! {
        #[doc = #trait_doc]
        pub trait #trait_name {
            type OutTuple;

            #[doc = #trait_doc]
            fn #fn_name(self) -> Self::OutTuple;
        }

        #(#impls)*
    };
    tks
}

fn gen_split_at_n_impl_size(ctx: &Ctx, trait_name: &Ident, fn_name: &Ident, n: usize, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let tks = match (n, size) {
        (_, 0) => quote! {},
        (1, 1) => quote! {},
        _ if size == n => quote! {},
        _ => {
            let l_part = &nts[..n];
            let r_part = &nts[n..];
            let l_part = tif! { l_part.len() == 1 => quote! { #(#l_part),* } ; quote! { (#(#l_part),*) } };
            let r_part = tif! { r_part.len() == 1 => quote! { #(#r_part),* } ; quote! { (#(#r_part),*) } };

            let lit_l_part = &size_lits[..n];
            let lit_r_part = &size_lits[n..];
            let lit_l_part = tif! { lit_l_part.len() == 1 => quote! { #(self.#lit_l_part),* } ; quote! { (#(self.#lit_l_part),*) } };
            let lit_r_part = tif! { lit_r_part.len() == 1 => quote! { #(self.#lit_r_part),* } ; quote! { (#(self.#lit_r_part),*) } };

            quote! {
                impl<#(#nts),*> #trait_name for (#(#nts),*) {
                    type OutTuple = (#l_part, #r_part);

                    fn #fn_name(self) -> Self::OutTuple {
                        (#lit_l_part, #lit_r_part)
                    }
                }
            }
        }
    };
    tks
}

fn gen_split_to_tuple_at(ctx: &Ctx, out_dir: &Path) {
    let items = (1..32usize).map(|i| gen_split_to_tuple_at_n(ctx, i));
    let tks = quote! {
        #(#items)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("split_to_tuple_at.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_split_to_tuple_at_n(ctx: &Ctx, n: usize) -> TokenStream {
    let trait_name = format_ident!("TupleSplitToTupleAt{}", n);
    let fn_name = format_ident!("split_to_tuple_at_{}", n);
    let trait_doc = format!("Split to tuple at {}", n);

    let impls = (n..33usize).map(|i| gen_split_to_tuple_at_n_impl_size(ctx, &trait_name, &fn_name, n, i));

    let tks = quote! {
        #[doc = #trait_doc]
        pub trait #trait_name {
            type OutTuple;

            #[doc = #trait_doc]
            fn #fn_name(self) -> Self::OutTuple;
        }

        #(#impls)*
    };
    tks
}

fn gen_split_to_tuple_at_n_impl_size(ctx: &Ctx, trait_name: &Ident, fn_name: &Ident, n: usize, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let tks = match (n, size) {
        (_, 0) => quote! {},
        (1, 1) => quote! {},
        _ if size == n => quote! {},
        _ => {
            let l_part = &nts[..n];
            let r_part = &nts[n..];
            let l_part = tif! { l_part.len() == 1 => quote! { (#(#l_part),*,) } ; quote! { (#(#l_part),*) } };
            let r_part = tif! { r_part.len() == 1 => quote! { (#(#r_part),*,) } ; quote! { (#(#r_part),*) } };

            let lit_l_part = &size_lits[..n];
            let lit_r_part = &size_lits[n..];
            let lit_l_part = tif! { lit_l_part.len() == 1 => quote! { (#(self.#lit_l_part),*,) } ; quote! { (#(self.#lit_l_part),*) } };
            let lit_r_part = tif! { lit_r_part.len() == 1 => quote! { (#(self.#lit_r_part),*,) } ; quote! { (#(self.#lit_r_part),*) } };

            quote! {
                impl<#(#nts),*> #trait_name for (#(#nts),*) {
                    type OutTuple = (#l_part, #r_part);

                    fn #fn_name(self) -> Self::OutTuple {
                        (#lit_l_part, #lit_r_part)
                    }
                }
            }
        }
    };
    tks
}

fn gen_transpose(ctx: &Ctx, out_dir: &Path) {
    let none_impl = ctx.size_lits.iter().map(|_| quote! { None }).collect::<Vec<_>>();
    let items_1 = (2..33usize).map(|i| gen_transpose_size_option_1(ctx, i, &none_impl[0..i]));
    let items_2 = (2..33usize).map(|i| gen_transpose_size_option_2(ctx, i));
    let items_3 = (2..33usize).map(|i| gen_transpose_size_result(ctx, i));

    let tks = quote! { #(#items_1)* #(#items_2)* #(#items_3)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("transpose.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_transpose_size_option_1(ctx: &Ctx, size: usize, none_impl: &[TokenStream]) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let i = &ctx.size_lits[0..size];
    let tks = quote! {
        impl<#(#nts,)*> TupleTranspose for Option<(#(#nts,)*)> {
            type Output = (#(Option<#nts>),*);

            fn transpose(self) -> Self::Output {
                match self {
                    Some(v) => (#(Some(v.#i)),*),
                    None => (#(#none_impl),*),
                }
            }
        }
    };
    tks
}

fn gen_transpose_size_option_2(ctx: &Ctx, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let nvs = &ctx.nvs[0..size];
    let tks = quote! {
        impl<#(#nts),*> TupleTranspose for (#(Option<#nts>),*) {
            type Output = Option<(#(#nts),*)>;

            fn transpose(self) -> Self::Output {
                match self {
                    (#(Some(#nvs)),*) => Some((#(#nvs),*)),
                    _ => None,
                }
            }
        }
    };
    tks
}

fn gen_transpose_size_result(ctx: &Ctx, size: usize) -> TokenStream {
    // let nts = &ctx.nts[0..size];
    // let ents = (0..size).map(|i| format_ident!("E{}", i)).collect::<Vec<_>>();
    // let nvs = &ctx.nvs[0..size];
    // let f_names = (0..size).map(|i| format_ident!("f{}", i)).collect::<Vec<_>>();
    // let gat_impl_trait_name = format_ident!("TupleTransposeResult{}", size);
    // let map_err_trait_name = format_ident!("TupleTransposeResultMapErr{}", size);
    // let map_err_params = (0..size)
    //     .enumerate()
    //     .map(|(i, _)| {
    //         let f_name = &f_names[i];
    //         let ent = &ents[i];
    //         quote! { #f_name: impl FnOnce(#ent) -> Eo }
    //     })
    //     .collect::<Vec<_>>();
    // let map_err_matchs = (0..size)
    //     .enumerate()
    //     .map(|(i, _)| {
    //         let nv = &nvs[i];
    //         let f_name = &f_names[i];
    //         quote! { match #nv {
    //             Ok(v) => v,
    //             Err(e) => Err(#f_name(e))?,
    //         } }
    //     })
    //     .collect::<Vec<_>>();
    let tks = quote! {
        // impl<E, #(#nts),*> TupleTransposeResultSameError for (#(Result<#nts, E>),*) {
        //     type OutTuple = Result<(#(#nts),*), E>;

        //     fn transpose_same_error(self) -> Self::OutTuple {
        //         let (#(#nvs),*) = self;
        //         Ok((#(#nvs?),*))
        //     }
        // }

        // /// Transposes for Result
        // pub trait #gat_impl_trait_name<#(#ents),*> {
        //     type OutTuple<Eo>;

        //     /// Transposes for Result
        //     fn transpose<Eo: #(From<#ents>)+*>(self) -> Self::OutTuple<Eo>;
        // }

        // impl<#(#ents, #nts),*> #gat_impl_trait_name<#(#ents),*> for (#(Result<#nts, #ents>),*) {
        //     type OutTuple<Eo> = Result<(#(#nts),*), Eo>;

        //     fn transpose<Eo: #(From<#ents>)+*>(self) -> Self::OutTuple<Eo> {
        //         let (#(#nvs),*) = self;
        //         Ok((#(#nvs?),*))
        //     }
        // }

        // /// Transposes for Result
        // pub trait #map_err_trait_name<#(#ents),*> {
        //     type OutTuple<Eo>;

        //     /// Transposes for Result
        //     fn transpose_map_err<Eo>(self, #(#map_err_params),*) -> Self::OutTuple<Eo>;
        // }

        // impl<#(#ents, #nts),*> #map_err_trait_name<#(#ents),*> for (#(Result<#nts, #ents>),*) {
        //     type OutTuple<Eo> = Result<(#(#nts),*), Eo>;

        //     fn transpose_map_err<Eo>(self, #(#map_err_params),*) -> Self::OutTuple<Eo> {
        //         let (#(#nvs),*) = self;
        //         Ok((#(#map_err_matchs),*))
        //     }
        // }
    };
    tks
}

fn gen_flatten(ctx: &Ctx, out_dir: &Path) {
    let et = quote! { () };
    let ets = ctx.nts.iter().map(|_| &et).collect::<Vec<_>>();

    let item_0s = (2..33usize).map(|i| gen_flatten_size_0(&ets[0..i]));

    let items = (2..33).map(|i| gen_flatten_size(ctx, i));

    let tks = quote! {
        #(#item_0s)*
        #(#items)*
    };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("flatten.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_flatten_size_0(ets: &[&TokenStream]) -> TokenStream {
    let tks = quote! {
        impl TupleFlatten for (#(#ets),*) {
            type OutTuple = ();

            fn flatten(self) -> Self::OutTuple {
                ()
            }
        }
    };
    tks
}

fn gen_flatten_size(ctx: &Ctx, size: usize) -> TokenStream {
    let max = (32 / size) + 1;
    let items = (1..33.min(max)).map(|i| gen_flatten_size_n(ctx, size, i));
    let tks = quote! {
        #(#items)*
    };
    tks
}

fn gen_flatten_size_n(ctx: &Ctx, size: usize, n: usize) -> TokenStream {
    let nct = tif! { n == 1 => quote! { , } ; quote! {} };
    let nts = &ctx.nts[0..size * n];
    let nnts = (0..size * n).step_by(n).map(|i| &nts[i..i + n]).map(|nts| quote! { (#(#nts),*#nct) });
    let nnimpl = ctx.size_lits[0..size].iter().flat_map(|i| ctx.size_lits[0..n].iter().map(move |n| quote! { (self.#i).#n }));
    let tks = quote! {
        impl<#(#nts),*> TupleFlatten for (#(#nnts),*) {
            type OutTuple = (#(#nts),*);

            fn flatten(self) -> Self::OutTuple {
                (#(#nnimpl),*)
            }
        }
    };
    tks
}

fn gen_clone(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_clone_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("clone.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_clone_size(ctx: &Ctx, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let tks = quote! {
        impl<'a, #(#nts: Clone),*> TupleCloned for (#(&'a #nts),*) {
            type Output = (#(#nts),*);

            fn cloned(self) -> Self::Output {
                (#(self.#size_lits.clone()),*)
            }
        }

        impl<'a, #(#nts: Clone),*> TupleCloned for (#(&'a mut #nts),*) {
            type Output = (#(#nts),*);

            fn cloned(self) -> Self::Output {
                (#(self.#size_lits.clone()),*)
            }
        }

        impl<'a, #(#nts: Copy),*> TupleCopied for (#(&'a #nts),*) {
            type Output = (#(#nts),*);

            fn copied(self) -> Self::Output {
                (#(*self.#size_lits),*)
            }
        }

        impl<'a, #(#nts: Copy),*> TupleCopied for (#(&'a mut #nts),*) {
            type Output = (#(#nts),*);

            fn copied(self) -> Self::Output {
                (#(*self.#size_lits),*)
            }
        }
    };
    tks
}

fn gen_call(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_call_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("tuple_call.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_call_size(ctx: &Ctx, size: usize) -> TokenStream {
    let name = format_ident!("Tuple{}Call", size);
    let nts = &ctx.nts[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let tks = quote! {
        pub trait #name<#(#nts),*> {
            fn call<F: FnOnce(#(#nts),*) -> O, O>(self, f: F) -> O;
        }
        impl<#(#nts),*> #name<#(#nts),*> for (#(#nts),*) {
            fn call<F: FnOnce(#(#nts),*) -> O, O>(self, f: F) -> O {
                f(#(self.#size_lits),*)
            }
        }
    };
    tks
}

fn gen_apply_tuple(ctx: &Ctx, out_dir: &Path) {
    let items = (0..33usize).map(|i| gen_apply_tuple_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("apply_tuple.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_apply_tuple_size(ctx: &Ctx, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let nvs = &ctx.nvs[0..size];
    let tks = quote! {
        impl<R, #(#nts),*> TupleFnMeta<R> for (#(#nts,)*) {
            type DynFnOnce = dyn FnOnce(#(#nts),*) -> R;
            type DynFnMut = dyn FnMut(#(#nts),*) -> R;
            type DynFn = dyn Fn(#(#nts),*) -> R;
            type FnPtr = fn(#(#nts),*) -> R;
        }

        impl<F: FnOnce(#(#nts),*) -> R, R, #(#nts),*> ApplyTupleMeta<(#(#nts,)*)> for F {
            type Output = R;
            type DynFnOnce = dyn FnOnce(#(#nts),*) -> R;
            type DynFnMut = dyn FnMut(#(#nts),*) -> R;
            type DynFn = dyn Fn(#(#nts),*) -> R;
            type FnPtr = fn(#(#nts),*) -> R;
        }

        impl<F: FnOnce(#(#nts),*) -> R, R, #(#nts),*> ApplyTupleOnce<(#(#nts,)*)> for F {
            type Output = R;

            fn apply_tuple_once(self, (#(#nvs,)*): (#(#nts,)*)) -> Self::Output {
                self(#(#nvs),*)
            }
        }

        impl<F: FnMut(#(#nts),*) -> R, R, #(#nts),*> ApplyTupleMut<(#(#nts,)*)> for F {
            fn apply_tuple_mut(&mut self, (#(#nvs,)*): (#(#nts,)*)) -> Self::Output {
                self(#(#nvs),*)
            }
        }

        impl<F: Fn(#(#nts),*) -> R, R, #(#nts),*> ApplyTuple<(#(#nts,)*)> for F {
            fn apply_tuple(&self, (#(#nvs,)*): (#(#nts,)*)) -> Self::Output {
                self(#(#nvs),*)
            }
        }
    };
    tks
}

fn gen_capt(ctx: &Ctx, out_dir: &Path) {
    let items = (1..33usize).map(|i| gen_capt_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("capt.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_capt_size(ctx: &Ctx, size: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let nvs = &ctx.nvs[0..size];
    let name = format_ident!("capt_{}", size);
    let name_mut = format_ident!("capt_mut_{}", size);
    let name_once = format_ident!("capt_once_{}", size);
    let tks = quote! {
        #[doc = "Manually capture the variables required by the closure"]
        pub fn #name<C, #(#nts),*, R, F: Fn(&C, #(#nts),*) -> R>(c: C, f: F) -> impl Fn(#(#nts),*) -> R {
            move |#(#nvs,)*| f(&c, #(#nvs,)*)
        }

        #[doc = "Manually capture the variables required by the closure"]
        pub fn #name_mut<C, #(#nts),*, R, F: FnMut(&mut C, #(#nts),*) -> R>(mut c: C, mut f: F) -> impl FnMut(#(#nts),*) -> R {
            move |#(#nvs,)*| f(&mut c, #(#nvs,)*)
        }

        #[doc = "Manually capture the variables required by the closure"]
        pub fn #name_once<C, #(#nts),*, R, F: FnOnce(C, #(#nts),*) -> R>(c: C, f: F) -> impl FnOnce(#(#nts),*) -> R {
            move |#(#nvs,)*| f(c, #(#nvs,)*)
        }

    };
    tks
}

fn gen_permutations(ctx: &Ctx, out_dir: &Path) {
    let items = (2..4usize).into_iter().map(|i| gen_permutations_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("permutations.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_permutations_size(ctx: &Ctx, size: usize) -> TokenStream {
    let trait_name = format_ident!("TuplePermutations{}", size);
    let fn_name = format_ident!("permutations_{}", size);

    let impls = (size..33usize).into_iter().filter(|i| i.pow(size as u32) - i < 33).map(|i| gen_permutations_impl_n(ctx, size, i, &trait_name, &fn_name));

    let doc = format!("Permutation by {size}");

    let tks = quote! {
        #[doc = #doc]
        pub trait #trait_name {
            type Output;

            #[doc = #doc]
            fn #fn_name(self) -> Self::Output;
        }

        #(#impls)*
    };
    tks
}

fn gen_permutations_impl_n(ctx: &Ctx, size: usize, n: usize, trait_name: &Ident, fn_name: &Ident) -> TokenStream {
    let nts = &ctx.nts[0..n];
    let size_lits = &ctx.size_lits[0..n];

    let output_types = nts
        .iter()
        .permutations(size)
        .map(|v| {
            quote! {
                (#(#v,)*)
            }
        })
        .collect::<Vec<_>>();

    let mut set = std::collections::HashSet::<usize>::new();
    let output_impls = size_lits
        .iter()
        .enumerate()
        .permutations(size)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|v| {
            let v = v.into_iter().map(|(i, l)| {
                if set.contains(&i) {
                    quote! {
                        self.#l.clone()
                    }
                } else {
                    set.insert(i);
                    quote! {
                        self.#l
                    }
                }
            });
            quote! {
                (#(#v,)*)
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    let tks = quote! {
        impl<#(#nts:Clone),*> #trait_name for (#(#nts,)*) {
            type Output = (#(#output_types,)*);

            fn #fn_name(self) -> Self::Output {
                (#(#output_impls,)*)
            }
        }
    };
    tks
}

fn gen_combinations(ctx: &Ctx, out_dir: &Path) {
    let items = (2..7usize).into_iter().map(|i| gen_combinations_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("combinations.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_combinations_size(ctx: &Ctx, size: usize) -> TokenStream {
    let trait_name = format_ident!("TupleCombinations{}", size);
    let fn_name = format_ident!("combinations_{}", size);

    let impls = (size..13usize)
        .into_iter()
        .filter(|n| {
            let count = count_combinations(*n, size);
            count < 33 && count > 1
        })
        .map(|i| gen_combinations_impl_n(ctx, size, i, &trait_name, &fn_name));

    let doc = format!("Combinations by {size}");

    let tks = quote! {
        #[doc = #doc]
        pub trait #trait_name {
            type Output;

            #[doc = #doc]
            fn #fn_name(self) -> Self::Output;
        }

        #(#impls)*
    };
    tks
}

fn gen_combinations_impl_n(ctx: &Ctx, size: usize, n: usize, trait_name: &Ident, fn_name: &Ident) -> TokenStream {
    let nts = &ctx.nts[0..n];
    let size_lits = &ctx.size_lits[0..n];

    let output_types = nts
        .iter()
        .combinations(size)
        .map(|v| {
            quote! {
                (#(#v,)*)
            }
        })
        .collect::<Vec<_>>();

    let mut set = std::collections::HashSet::<usize>::new();
    let output_impls = size_lits
        .iter()
        .enumerate()
        .combinations(size)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .map(|v| {
            let v = v.into_iter().map(|(i, l)| {
                if set.contains(&i) {
                    quote! {
                        self.#l.clone()
                    }
                } else {
                    set.insert(i);
                    quote! {
                        self.#l
                    }
                }
            });
            quote! {
                (#(#v,)*)
            }
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev();

    let tks = quote! {
        impl<#(#nts:Clone),*> #trait_name for (#(#nts,)*) {
            type Output = (#(#output_types,)*);

            fn #fn_name(self) -> Self::Output {
                (#(#output_impls,)*)
            }
        }
    };
    tks
}

fn count_combinations(n: usize, k: usize) -> usize {
    if k > n {
        return 0;
    }
    if k == 0 || k == n {
        return 1;
    }
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in (1..=k.min(i)).rev() {
            dp[j] += dp[j - 1];
        }
    }
    dp[k]
}

fn gen_uniform_map(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_uniform_map_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("uniform_map.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_uniform_map_size(ctx: &Ctx, size: usize) -> TokenStream {
    let ts = &ctx.ts[0..size];
    let us = &ctx.us[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let nts = &ctx.nts[0..size];
    let ntfs = &ctx.ntfs[0..size];

    let tfs_once = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: FnOnce(#nt) -> U
        }
    });

    let tfs_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: FnMut(#nt) -> U
        }
    });

    let tfs = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: Fn(#nt) -> U
        }
    });

    let tfs_once_ref = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: FnOnce(&#nt) -> U
        }
    });

    let tfs_mut_ref = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: FnMut(&#nt) -> U
        }
    });

    let tfs_ref = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: Fn(&#nt) -> U
        }
    });

    let tfs_once_ref_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: FnOnce(&mut #nt) -> U
        }
    });

    let tfs_mut_ref_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: FnMut(&mut #nt) -> U
        }
    });

    let tfs_ref_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: Fn(&mut #nt) -> U
        }
    });

    let tks = quote! {
        impl<U, T, F: FnMut(T) -> U> TupleUniformMapper<(#(#ts),*), U> for F {
            type Output = (#(#us),*);

            fn apply_uniform_map(mut self, input: (#(#ts),*)) -> Self::Output {
                (#((&mut self)(input.#size_lits)),*)
            }
        }

        impl<U, T, F: FnMut(&T) -> U> TupleUniformMapper<&(#(#ts),*), U> for F {
            type Output = (#(#us),*);

            fn apply_uniform_map(mut self, input: &(#(#ts),*)) -> Self::Output {
                (#((&mut self)(&input.#size_lits)),*)
            }
        }

        impl<U, T, F: FnMut(&mut T) -> U> TupleUniformMapper<&mut (#(#ts),*), U> for F {
            type Output = (#(#us),*);

            fn apply_uniform_map(mut self, input: &mut (#(#ts),*)) -> Self::Output {
                (#((&mut self)(&mut input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_once),* > TupleUniformMapper<(#(#nts),*), U> for (#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_mut),* > TupleUniformMapper<(#(#nts),*), U> for &mut (#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs),* > TupleUniformMapper<(#(#nts),*), U> for &(#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_once_ref),* > TupleUniformMapper<&(#(#nts),*), U> for (#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: &(#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(&input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_mut_ref),* > TupleUniformMapper<&(#(#nts),*), U> for &mut (#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: &(#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(&input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_ref),* > TupleUniformMapper<&(#(#nts),*), U> for &(#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: &(#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(&input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_once_ref_mut),* > TupleUniformMapper<&mut (#(#nts),*), U> for (#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: &mut (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(&mut input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_mut_ref_mut),* > TupleUniformMapper<&mut (#(#nts),*), U> for &mut (#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: &mut (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(&mut input.#size_lits)),*)
            }
        }

        impl<U, #(#nts),*, #(#tfs_ref_mut),* > TupleUniformMapper<&mut (#(#nts),*), U> for &(#(#ntfs),*)
        {
            type Output = (#(#us),*);

            fn apply_uniform_map(self, input: &mut (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(&mut input.#size_lits)),*)
            }
        }
    };
    tks
}

fn gen_uniform_map_by(ctx: &Ctx, out_dir: &Path) {
    let items = (2..33usize).map(|i| gen_uniform_map_by_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("uniform_map_by.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_uniform_map_by_size(ctx: &Ctx, size: usize) -> TokenStream {
    let ts = &ctx.ts[0..size];
    let us = &ctx.us[0..size];
    let size_lits = &ctx.size_lits[0..size];

    let nts = &ctx.nts[0..size];
    let ntfs = &ctx.ntfs[0..size];

    let tfs_once = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> FnOnce(<A as RefOrMut<'s>>::Target, #nt) -> U
        }
    });

    let tfs_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> FnMut(<A as RefOrMut<'s>>::Target, #nt) -> U
        }
    });

    let tfs = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> Fn(<A as RefOrMut<'s>>::Target, #nt) -> U
        }
    });

    let tfs_once_ref = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> FnOnce(<A as RefOrMut<'s>>::Target, &#nt) -> U
        }
    });

    let tfs_mut_ref = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> FnMut(<A as RefOrMut<'s>>::Target, &#nt) -> U
        }
    });

    let tfs_ref = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> Fn(<A as RefOrMut<'s>>::Target, &#nt) -> U
        }
    });

    let tfs_once_ref_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> FnOnce(<A as RefOrMut<'s>>::Target, &mut #nt) -> U
        }
    });

    let tfs_mut_ref_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> FnMut(<A as RefOrMut<'s>>::Target, &mut #nt) -> U
        }
    });

    let tfs_ref_mut = ntfs.iter().enumerate().map(|(i, ntf)| {
        let nt = &nts[i];

        quote! {
            #ntf: for<'s> Fn(<A as RefOrMut<'s>>::Target, &mut #nt) -> U
        }
    });

    let tks = quote! {
        impl<A, U, T, F: for<'s> FnMut(<A as RefOrMut<'s>>::Target, T) -> U> TupleUniformMapperBy<(#(#ts),*), U, A> for F
        where
            A: for<'s> RefOrMut<'s>,{
            type Output = (#(#us),*);

            fn apply_uniform_map_by(mut self, mut arg: A, input: (#(#ts),*)) -> Self::Output {
                (#((&mut self)(arg.pass(), input.#size_lits)),*)
            }
        }

        impl<A, U, T, F: for<'s> FnMut(<A as RefOrMut<'s>>::Target, &T) -> U> TupleUniformMapperBy<&(#(#ts),*), U, A> for F
        where
            A: for<'s> RefOrMut<'s>,{
            type Output = (#(#us),*);

            fn apply_uniform_map_by(mut self, mut arg: A, input: &(#(#ts),*)) -> Self::Output {
                (#((&mut self)(arg.pass(), &input.#size_lits)),*)
            }
        }

        impl<A, U, T, F: for<'s> FnMut(<A as RefOrMut<'s>>::Target, &mut T) -> U> TupleUniformMapperBy<&mut (#(#ts),*), U, A> for F
        where
            A: for<'s> RefOrMut<'s>,{
            type Output = (#(#us),*);

            fn apply_uniform_map_by(mut self, mut arg: A, input: &mut (#(#ts),*)) -> Self::Output {
                (#((&mut self)(arg.pass(), &mut input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_once),* > TupleUniformMapperBy<(#(#nts),*), U, A> for (#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_mut),* > TupleUniformMapperBy<(#(#nts),*), U, A> for &mut (#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs),* > TupleUniformMapperBy<(#(#nts),*), U, A> for &(#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_once_ref),* > TupleUniformMapperBy<&(#(#nts),*), U, A> for (#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: &(#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), &input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_mut_ref),* > TupleUniformMapperBy<&(#(#nts),*), U, A> for &mut (#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: &(#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), &input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_ref),* > TupleUniformMapperBy<&(#(#nts),*), U, A> for &(#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: &(#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), &input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_once_ref_mut),* > TupleUniformMapperBy<&mut (#(#nts),*), U, A> for (#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: &mut (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), &mut input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_mut_ref_mut),* > TupleUniformMapperBy<&mut (#(#nts),*), U, A> for &mut (#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: &mut (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), &mut input.#size_lits)),*)
            }
        }

        impl<A, U, #(#nts),*, #(#tfs_ref_mut),* > TupleUniformMapperBy<&mut (#(#nts),*), U, A> for &(#(#ntfs),*)
        where
            A: for<'s> RefOrMut<'s>,
        {
            type Output = (#(#us),*);

            fn apply_uniform_map_by(self, mut arg: A, input: &mut (#(#nts),*)) -> Self::Output {
                (#((self.#size_lits)(arg.pass(), &mut input.#size_lits)),*)
            }
        }
    };
    tks
}

fn gen_get_dyn(ctx: &Ctx, out_dir: &Path) {
    let items = (1..33usize).map(|i| gen_get_dyn_size(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("get_dyn.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_get_dyn_size(ctx: &Ctx, size: usize) -> TokenStream {
    let ts = &ctx.ts[0..size];
    let size_lits = &ctx.size_lits[0..size];
    let take_some: Vec<_> = size_lits.iter().map(|i| quote! { #i => Some(&self.#i), }).collect();
    let take_mut_some: Vec<_> = size_lits.iter().map(|i| quote! { #i => Some(&mut self.#i), }).collect();

    let tks = quote! {
        impl<T> TupleDynamicGet for (#(#ts,)*) {
            type Output = T;

            fn dyn_get(&self, index: usize) -> Option<&Self::Output> {
                match index {
                    #(#take_some)*
                    _ => None,
                }
            }
        }

        impl<T> TupleDynamicGetMut for (#(#ts,)*) {
            fn dyn_get_mut(&mut self, index: usize) -> Option<&mut Self::Output> {
                match index {
                    #(#take_mut_some)*
                    _ => None,
                }
            }
        }

        impl<T> TupleDynamicSwap for (#(#ts,)*) {
            fn dyn_swap(&mut self, a: usize, b: usize) -> bool {
                if a >= #size || b >= #size {
                    return false;
                }
                if a == b {
                    return true;
                }
                unsafe {
                    let a =  self.dyn_get_mut(a).unwrap() as *mut _;
                    let b =  self.dyn_get_mut(b).unwrap() as *mut _;
                    core::mem::swap(&mut *a, &mut *b);
                }
                true
            }
        }
    };
    tks
}

fn gen_get_const(ctx: &Ctx, out_dir: &Path) {
    let items = (1..33usize).map(|i| gen_get_size_const(ctx, i));
    let tks = quote! { #(#items)* };
    let mut code = tks.to_string();
    code.insert_str(0, AUTO_GEN_TIP);
    let dest_path = Path::new(out_dir).join("get_const.rs");
    fs::write(&dest_path, code).unwrap();
}

fn gen_get_size_const(ctx: &Ctx, size: usize) -> TokenStream {
    let tks = (0..size).into_iter().map(|n| gen_get_size_const_n(ctx, size, n));
    quote! {
     #(#tks)*
    }
}

fn gen_get_size_const_n(ctx: &Ctx, size: usize, n: usize) -> TokenStream {
    let nts = &ctx.nts[0..size];
    let nt = &ctx.nts[n];
    let n_lit = &ctx.size_lits[n];

    let tks = quote! {
        impl<#(#nts,)*> TupleGetN<#n_lit> for (#(#nts),*,) {
            type Output = #nt;

            fn get_n(&self) -> &Self::Output
            {
                &self.#n_lit
            }
        }
        impl<#(#nts,)*> TupleGetMutN<#n_lit> for (#(#nts),*,) {
            fn get_n_mut(&mut self) -> &mut Self::Output
            {
                &mut self.#n_lit
            }
        }
    };
    tks
}
