// Generated macro for impl_311 (impl)
macro_rules! Depcrate_deimpl_311 {
() => {
// Module: crate::de
// Provides: {"impl_311"}
// Dependencies: {}
impl < 'a > ToTokens for DeTypeGenerics < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { de_type_generics_to_tokens (self . 0 . generics . clone () , & self . 0 . borrowed , tokens) ; } }
};
}
