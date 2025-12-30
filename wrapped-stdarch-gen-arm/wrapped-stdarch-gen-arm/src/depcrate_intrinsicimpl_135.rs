// Generated macro for impl_135 (impl)
macro_rules! Depcrate_intrinsicimpl_135 {
() => {
// Module: crate::intrinsic
// Provides: {"impl_135"}
// Dependencies: {}
impl ToTokens for Argument { fn to_tokens (& self , tokens : & mut TokenStream) { if let AccessLevel :: RW = & self . rw { tokens . append (format_ident ! ("mut")) } let (name , kind) = (format_ident ! ("{}" , self . name . to_string ()) , & self . kind) ; tokens . append_all (quote ! { # name : # kind }) } }
};
}
