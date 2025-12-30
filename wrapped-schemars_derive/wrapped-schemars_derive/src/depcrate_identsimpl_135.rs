// Generated macro for impl_135 (impl)
macro_rules! Depcrate_identsimpl_135 {
() => {
// Module: crate::idents
// Provides: {"impl_135"}
// Dependencies: {}
impl quote :: ToTokens for ConstIdent { fn to_tokens (& self , tokens : & mut TokenStream) { let ident = Ident :: new (self . 0 , Span :: call_site ()) ; tokens . append (ident) ; } }
};
}
