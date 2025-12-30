// Generated macro for impl_73 (impl)
macro_rules! Depcrate_genericsimpl_73 {
() => {
// Module: crate::generics
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg (feature = "printing")] impl Generics { # [doc = " Split a type's generics into the pieces required for impl'ing a trait"] # [doc = " for that type."] # [doc = ""] # [doc = " ```"] # [doc = " # extern crate syn;"] # [doc = " # #[macro_use]"] # [doc = " # extern crate quote;"] # [doc = " # fn main() {"] # [doc = " # let generics: syn::Generics = Default::default();"] # [doc = " # let name = syn::Ident::from(\"MyType\");"] # [doc = " let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();"] # [doc = " quote! {"] # [doc = "     impl #impl_generics MyTrait for #name #ty_generics #where_clause {"] # [doc = "         // ..."] # [doc = "     }"] # [doc = " }"] # [doc = " # ;"] # [doc = " # }"] # [doc = " ```"] pub fn split_for_impl (& self) -> (ImplGenerics , TyGenerics , & WhereClause) { (ImplGenerics (self) , TyGenerics (self) , & self . where_clause) } }
};
}
