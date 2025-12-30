// Generated macro for impl_30 (impl)
macro_rules! Depcrate_entryimpl_30 {
() => {
// Module: crate::entry
// Provides: {"impl_30"}
// Dependencies: {}
impl ToTokens for Body < '_ > { fn to_tokens (& self , tokens : & mut proc_macro2 :: TokenStream) { self . brace_token . surround (tokens , | tokens | { for stmt in self . stmts { stmt . to_tokens (tokens) ; } }) ; } }
};
}
