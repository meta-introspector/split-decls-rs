// Generated macro for impl_324 (impl)
macro_rules! Depcrate_wildstringimpl_324 {
() => {
// Module: crate::wildstring
// Provides: {"impl_324"}
// Dependencies: {}
impl ToTokens for WildString { fn to_tokens (& self , tokens : & mut TokenStream) { assert ! (! self . has_wildcards () , "cannot convert string with wildcards {self:?} to TokenStream") ; let str = self . to_string () ; tokens . append_all (quote ! { # str }) } }
};
}
