// Generated macro for impl_150 (impl)
macro_rules! Depcrate_intrinsicimpl_150 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_150"}
// Dependencies: {}
impl ToTokens for LLVMLink { fn to_tokens (& self , tokens : & mut TokenStream) { assert ! (self . signature . is_some () && self . links . is_some () , "expression {self:#?} was not built before calling to_tokens") ; let signature = self . signature . as_ref () . unwrap () ; let links = self . links . as_ref () . unwrap () ; tokens . append_all (quote ! { unsafe extern "unadjusted" { # (# links) * # signature ; } }) } }
};
}
