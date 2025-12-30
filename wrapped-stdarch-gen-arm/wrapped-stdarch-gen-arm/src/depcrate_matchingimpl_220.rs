// Generated macro for impl_220 (impl)
macro_rules! Depcrate_matchingimpl_220 {
() => {
// Module: crate::matching
// Provides: {"impl_220"}
// Dependencies: {}
impl < T : fmt :: Debug + ToTokens > ToTokens for KindMatchable < T > { fn to_tokens (& self , tokens : & mut TokenStream) { self . as_ref () . to_tokens (tokens) } }
};
}
