// Generated macro for cfg (function)
macro_rules! Depcrate_expandcfg {
() => {
// Module: crate::expand
// Provides: {"cfg"}
// Dependencies: {}
pub fn cfg (introducer : & str , args : TokenStream , input : TokenStream) -> TokenStream { try_cfg (introducer , args , input) . unwrap_or_else (Error :: into_compile_error) }
};
}
