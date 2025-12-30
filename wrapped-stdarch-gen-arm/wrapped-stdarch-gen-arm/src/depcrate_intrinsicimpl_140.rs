// Generated macro for impl_140 (impl)
macro_rules! Depcrate_intrinsicimpl_140 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_140"}
// Dependencies: {}
impl ToTokens for StaticDefinition { fn to_tokens (& self , tokens : & mut TokenStream) { tokens . append_all (match self { StaticDefinition :: Constant (arg) => quote ! { const # arg } , StaticDefinition :: Generic (generic) => { let generic : TokenStream = generic . parse () . expect ("invalid Rust code") ; quote ! { # generic } } }) } }
};
}
