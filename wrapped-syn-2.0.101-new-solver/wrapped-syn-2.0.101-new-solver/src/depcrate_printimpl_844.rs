// Generated macro for impl_844 (impl)
macro_rules! Depcrate_printimpl_844 {
() => {
// Module: crate::print
// Provides: {"impl_844"}
// Dependencies: {}
impl < 'a , T > ToTokens for TokensOrDefault < 'a , T > where T : ToTokens + Default , { fn to_tokens (& self , tokens : & mut TokenStream) { match self . 0 { Some (t) => t . to_tokens (tokens) , None => T :: default () . to_tokens (tokens) , } } }
};
}
