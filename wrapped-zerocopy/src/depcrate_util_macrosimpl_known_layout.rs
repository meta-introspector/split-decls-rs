// Generated macro for impl_known_layout (macro)
macro_rules! Depcrate_util_macrosimpl_known_layout {
() => {
// Module: crate::util::macros
// Provides: {"impl_known_layout"}
// Dependencies: {}
# [doc = " Implements `KnownLayout` for a sized type."] macro_rules ! impl_known_layout { ($ (const $ constvar : ident : $ constty : ty , $ tyvar : ident $ (: ?$ optbound : ident) ? => $ ty : ty) ,* $ (,) ?) => { $ (impl_known_layout ! (@ inner const $ constvar : $ constty , $ tyvar $ (: ?$ optbound) ? => $ ty) ;) * } ; ($ ($ tyvar : ident $ (: ?$ optbound : ident) ? => $ ty : ty) ,* $ (,) ?) => { $ (impl_known_layout ! (@ inner , $ tyvar $ (: ?$ optbound) ? => $ ty) ;) * } ; ($ ($ (# [$ attrs : meta]) * $ ty : ty) ,*) => { $ (impl_known_layout ! (@ inner , => $ (# [$ attrs]) * $ ty) ;) * } ; (@ inner $ (const $ constvar : ident : $ constty : ty) ? , $ ($ tyvar : ident $ (: ?$ optbound : ident) ?) ? => $ (# [$ attrs : meta]) * $ ty : ty) => { const _ : () = { use core :: ptr :: NonNull ; # [allow (non_local_definitions)] $ (# [$ attrs]) * unsafe impl <$ ($ tyvar $ (: ?$ optbound) ?) ? $ (, const $ constvar : $ constty) ?> KnownLayout for $ ty { # [allow (clippy :: missing_inline_in_public_items)] # [cfg_attr (all (coverage_nightly , __ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS) , coverage (off))] fn only_derive_is_allowed_to_implement_this_trait () where Self : Sized { } type PointerMetadata = () ; type MaybeUninit = core :: mem :: MaybeUninit < Self >; const LAYOUT : crate :: DstLayout = crate :: DstLayout :: for_type ::<$ ty > () ; # [inline (always)] fn raw_from_ptr_len (bytes : NonNull < u8 >, _meta : ()) -> NonNull < Self > { bytes . cast ::< Self > () } # [inline (always)] fn pointer_to_metadata (_ptr : * mut Self) -> () { } } } ; } ; }
};
}
