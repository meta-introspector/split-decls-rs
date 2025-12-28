macro_rules! deps {
    () => {
        Trait!();
    };
}

macro_rules! derive_eq_inner {
    () => {
        deps!();
        fn derive_eq_inner (ast : & DeriveInput , _top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { let type_ident = & ast . ident ; let (impl_generics , ty_generics , where_clause) = ast . generics . split_for_impl () ; let where_predicates = where_clause . map (| clause | & clause . predicates) ; Ok (quote ! { # [allow (deprecated)] # [automatically_derived] impl # impl_generics # zerocopy_crate :: util :: macro_util :: core_reexport :: cmp :: PartialEq for # type_ident # ty_generics where Self : # zerocopy_crate :: IntoBytes + # zerocopy_crate :: Immutable , # where_predicates { fn eq (& self , other : & Self) -> bool { # zerocopy_crate :: util :: macro_util :: core_reexport :: cmp :: PartialEq :: eq (# zerocopy_crate :: IntoBytes :: as_bytes (self) , # zerocopy_crate :: IntoBytes :: as_bytes (other) ,) } } # [allow (deprecated)] # [automatically_derived] impl # impl_generics # zerocopy_crate :: util :: macro_util :: core_reexport :: cmp :: Eq for # type_ident # ty_generics where Self : # zerocopy_crate :: IntoBytes + # zerocopy_crate :: Immutable , # where_predicates { } }) }
    };
}

derive_eq_inner!();