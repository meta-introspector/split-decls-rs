// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < T : ToTokens > ToTokens for QuoteOption < T > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { tokens . append_all (match self . 0 { Some (ref t) => quote ! { :: std :: option :: Option :: Some (# t) } , None => quote ! { :: std :: option :: Option :: None } , }) ; } }
};
}
