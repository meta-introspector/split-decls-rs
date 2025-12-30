// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl ToTokens for BindStyle { fn to_tokens (& self , tokens : & mut TokenStream) { match self { BindStyle :: Move => { } BindStyle :: MoveMut => quote_spanned ! (Span :: call_site () => mut) . to_tokens (tokens) , BindStyle :: Ref => quote_spanned ! (Span :: call_site () => ref) . to_tokens (tokens) , BindStyle :: RefMut => quote_spanned ! (Span :: call_site () => ref mut) . to_tokens (tokens) , } } }
};
}
