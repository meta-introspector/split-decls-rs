// Generated macro for impl_99 (impl)
macro_rules! Depcrate_identimpl_99 {
() => {
// Module: crate::ident
// Provides: {"impl_99"}
// Dependencies: {}
impl Ident { # [doc = " Creates a new `Ident` from the structured items. This is mainly used"] # [doc = " by the parser to create `Ident`s from existing Rust source code."] # [doc = ""] # [doc = " Creating new `Ident`s programmatically is easier with `Ident::from`."] pub fn new (sym : Term , span : Span) -> Self { let s = sym . as_str () ; if s . is_empty () { panic ! ("ident is not allowed to be empty; use Option<Ident>") ; } if s . starts_with ('\'') { panic ! ("ident is not allowed to be a lifetime; use syn::Lifetime") ; } if s == "_" { panic ! ("`_` is not a valid ident; use syn::tokens::Underscore") ; } fn xid_ok (s : & str) -> bool { let mut chars = s . chars () ; let first = chars . next () . unwrap () ; if ! (UnicodeXID :: is_xid_start (first) || first == '_') { return false ; } for ch in chars { if ! UnicodeXID :: is_xid_continue (ch) { return false ; } } true } fn integer_ok (s : & str) -> bool { s . bytes () . all (| digit | digit >= b'0' && digit <= b'9') } if ! (xid_ok (s) || integer_ok (s)) { panic ! ("{:?} is not a valid ident" , s) ; } Ident { sym : sym , span : span , } } }
};
}
