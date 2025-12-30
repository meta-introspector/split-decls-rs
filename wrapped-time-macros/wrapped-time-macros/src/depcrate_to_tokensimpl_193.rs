// Generated macro for impl_193 (impl)
macro_rules! Depcrate_to_tokensimpl_193 {
() => {
// Module: crate::to_tokens
// Provides: {"impl_193"}
// Dependencies: {}
impl < T : ToTokenTree > ToTokenStream for T { fn append_to (self , ts : & mut TokenStream) { ts . extend ([self . into_token_tree ()]) } }
};
}
