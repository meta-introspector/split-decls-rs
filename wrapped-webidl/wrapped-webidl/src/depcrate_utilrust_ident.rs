// Generated macro for rust_ident (function)
macro_rules! Depcrate_utilrust_ident {
() => {
// Module: crate::util
// Provides: {"rust_ident"}
// Dependencies: {}
# [doc = " Create an `Ident`, possibly mangling it if it conflicts with a Rust keyword."] pub fn rust_ident (name : & str) -> Ident { if name . is_empty () { panic ! ("tried to create empty Ident (from \"\")") ; } else if is_rust_keyword (name) { Ident :: new (& format ! ("{name}_") , proc_macro2 :: Span :: call_site ()) } else if name == "async" { let ident = "r#async" . parse :: < proc_macro2 :: TokenStream > () . unwrap () . into_iter () . next () . unwrap () ; match ident { proc_macro2 :: TokenTree :: Ident (i) => i , _ => unreachable ! () , } } else if name . chars () . next () . unwrap () . is_ascii_digit () { Ident :: new (& format ! ("N{name}") , proc_macro2 :: Span :: call_site ()) } else { Ident :: new (name , proc_macro2 :: Span :: call_site ()) } }
};
}
