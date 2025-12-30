// Generated macro for raw_ident (function)
macro_rules! Depcrate_utilraw_ident {
() => {
// Module: crate::util
// Provides: {"raw_ident"}
// Dependencies: {}
# [doc = " Create an `Ident` without checking to see if it conflicts with a Rust"] # [doc = " keyword."] pub fn raw_ident (name : & str) -> Ident { Ident :: new (name , proc_macro2 :: Span :: call_site ()) }
};
}
