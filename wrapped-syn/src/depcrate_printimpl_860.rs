// Generated macro for impl_860 (impl)
macro_rules! Depcrate_printimpl_860 {
() => {
// Module: crate::print
// Provides: {"impl_860"}
// Dependencies: {}
impl < 'a , T > ToTokens for TokensOrDefault < 'a , T > where T : ToTokens + Default , { fn to_tokens (& self , tokens : & mut TokenStream) { match self . 0 { Some (t) => t . to_tokens (tokens) , None => T :: default () . to_tokens (tokens) , } } }
};
}
