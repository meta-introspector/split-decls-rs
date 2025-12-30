// Generated macro for impl_215 (impl)
macro_rules! Depcrate_matchingimpl_215 {
() => {
// Module: crate::matching
// Provides: {"impl_215"}
// Dependencies: {}
impl < T : fmt :: Debug + ToTokens > ToTokens for SizeMatchable < T > { fn to_tokens (& self , tokens : & mut TokenStream) { self . as_ref () . to_tokens (tokens) } }
};
}
