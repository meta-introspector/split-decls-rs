// Generated macro for impl_93 (impl)
macro_rules! Depcrate_internals_nameimpl_93 {
() => {
// Module: crate::internals::name
// Provides: {"impl_93"}
// Dependencies: {}
impl ToTokens for Name { fn to_tokens (& self , tokens : & mut TokenStream) { LitStr :: new (& self . value , self . span) . to_tokens (tokens) ; } }
};
}
