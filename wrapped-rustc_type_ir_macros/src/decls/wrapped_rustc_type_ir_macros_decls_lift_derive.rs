use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn lift_derive(mut s: synstructure::Structure<'_>) -> proc_macro2::TokenStream {
    if let syn::Data::Union(_) = s.ast().data {
        panic!("cannot derive on union")
    }
    if !s.ast().generics.type_params().any(|ty| ty.ident == "I") {
        s.add_impl_generic(parse_quote! {
            I
        });
    }
    s.add_bounds(synstructure::AddBounds::None);
    s.add_where_predicate(parse_quote! {
        I : Interner
    });
    s.add_impl_generic(parse_quote! {
        J
    });
    s.add_where_predicate(parse_quote! {
        J : Interner
    });
    let mut wc = vec![];
    s.bind_with(|_| synstructure::BindStyle::Move);
    let body_fold = s.each_variant(|vi| {
        let bindings = vi.bindings();
        vi.construct(|field, index| {
            let ty = field.ty.clone();
            let lifted_ty = lift(ty.clone());
            wc.push(parse_quote! {
                # ty : ::rustc_type_ir::lift::Lift < J, Lifted = # lifted_ty >
            });
            let bind = &bindings[index];
            quote! {
                # bind.lift_to_interner(interner) ?
            }
        })
    });
    for wc in wc {
        s.add_where_predicate(wc);
    }
    let (_, ty_generics, _) = s.ast().generics.split_for_impl();
    let name = s.ast().ident.clone();
    let self_ty: syn::Type = parse_quote! {
        # name # ty_generics
    };
    let lifted_ty = lift(self_ty);
    s.bound_impl(
        quote!(::rustc_type_ir::lift::Lift<J>),
        quote! {
            type Lifted = # lifted_ty; fn lift_to_interner(self, interner : J,) -> Option
            < Self::Lifted > { Some(match self { # body_fold }) }
        },
    )
}
