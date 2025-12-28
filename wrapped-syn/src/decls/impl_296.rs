macro_rules! deps {
    () => {
        Lifetimes!();
        TypeParams!();
        TypeParamsMut!();
        Punctuated!();
        ConstParamsMut!();
        ImplGenerics!();
        LifetimesMut!();
        TypeGenerics!();
        ConstParams!();
    };
}

macro_rules! impl_296 {
    () => {
        deps!();
        impl Generics { return_impl_trait ! { # [doc = " Iterator over the lifetime parameters in `self.params`."] pub fn lifetimes (& self) -> impl Iterator < Item = & LifetimeParam > [Lifetimes] { Lifetimes (self . params . iter ()) } } return_impl_trait ! { # [doc = " Iterator over the lifetime parameters in `self.params`."] pub fn lifetimes_mut (& mut self) -> impl Iterator < Item = & mut LifetimeParam > [LifetimesMut] { LifetimesMut (self . params . iter_mut ()) } } return_impl_trait ! { # [doc = " Iterator over the type parameters in `self.params`."] pub fn type_params (& self) -> impl Iterator < Item = & TypeParam > [TypeParams] { TypeParams (self . params . iter ()) } } return_impl_trait ! { # [doc = " Iterator over the type parameters in `self.params`."] pub fn type_params_mut (& mut self) -> impl Iterator < Item = & mut TypeParam > [TypeParamsMut] { TypeParamsMut (self . params . iter_mut ()) } } return_impl_trait ! { # [doc = " Iterator over the constant parameters in `self.params`."] pub fn const_params (& self) -> impl Iterator < Item = & ConstParam > [ConstParams] { ConstParams (self . params . iter ()) } } return_impl_trait ! { # [doc = " Iterator over the constant parameters in `self.params`."] pub fn const_params_mut (& mut self) -> impl Iterator < Item = & mut ConstParam > [ConstParamsMut] { ConstParamsMut (self . params . iter_mut ()) } } # [doc = " Initializes an empty `where`-clause if there is not one present already."] pub fn make_where_clause (& mut self) -> & mut WhereClause { self . where_clause . get_or_insert_with (| | WhereClause { where_token : < Token ! [where] > :: default () , predicates : Punctuated :: new () , }) } # [doc = " Split a type's generics into the pieces required for impl'ing a trait"] # [doc = " for that type."] # [doc = ""] # [doc = " ```"] # [doc = " # use proc_macro2::{Span, Ident};"] # [doc = " # use quote::quote;"] # [doc = " #"] # [doc = " # let generics: syn::Generics = Default::default();"] # [doc = " # let name = Ident::new(\"MyType\", Span::call_site());"] # [doc = " #"] # [doc = " let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();"] # [doc = " quote! {"] # [doc = "     impl #impl_generics MyTrait for #name #ty_generics #where_clause {"] # [doc = "         // ..."] # [doc = "     }"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] # [cfg (feature = "printing")] # [cfg_attr (docsrs , doc (cfg (feature = "printing")))] pub fn split_for_impl (& self) -> (ImplGenerics , TypeGenerics , Option < & WhereClause >) { (ImplGenerics (self) , TypeGenerics (self) , self . where_clause . as_ref () ,) } }
    };
}

impl_296!();