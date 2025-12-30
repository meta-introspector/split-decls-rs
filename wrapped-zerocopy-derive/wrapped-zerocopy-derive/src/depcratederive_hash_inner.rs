// Generated macro for derive_hash_inner (function)
macro_rules! Depcratederive_hash_inner {
() => {
// Module: crate
// Provides: {"derive_hash_inner"}
// Dependencies: {}
fn derive_hash_inner (ast : & DeriveInput , _top_level : Trait , zerocopy_crate : & Path ,) -> Result < TokenStream , Error > { let type_ident = & ast . ident ; let (impl_generics , ty_generics , where_clause) = ast . generics . split_for_impl () ; let where_predicates = where_clause . map (| clause | & clause . predicates) ; Ok (quote ! { # [allow (deprecated)] # [automatically_derived] impl # impl_generics # zerocopy_crate :: util :: macro_util :: core_reexport :: hash :: Hash for # type_ident # ty_generics where Self : # zerocopy_crate :: IntoBytes + # zerocopy_crate :: Immutable , # where_predicates { fn hash < H > (& self , state : & mut H) where H : # zerocopy_crate :: util :: macro_util :: core_reexport :: hash :: Hasher , { # zerocopy_crate :: util :: macro_util :: core_reexport :: hash :: Hasher :: write (state , # zerocopy_crate :: IntoBytes :: as_bytes (self)) } fn hash_slice < H > (data : & [Self] , state : & mut H) where H : # zerocopy_crate :: util :: macro_util :: core_reexport :: hash :: Hasher , { # zerocopy_crate :: util :: macro_util :: core_reexport :: hash :: Hasher :: write (state , # zerocopy_crate :: IntoBytes :: as_bytes (data)) } } }) }
};
}
