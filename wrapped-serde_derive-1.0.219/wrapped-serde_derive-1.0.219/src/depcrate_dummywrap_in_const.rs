// Generated macro for wrap_in_const (function)
macro_rules! Depcrate_dummywrap_in_const {
() => {
// Module: crate::dummy
// Provides: {"wrap_in_const"}
// Dependencies: {}
pub fn wrap_in_const (serde_path : Option < & syn :: Path > , code : TokenStream) -> TokenStream { let use_serde = match serde_path { Some (path) => quote ! { use # path as _serde ; } , None => quote ! { # [allow (unused_extern_crates , clippy :: useless_attribute)] extern crate serde as _serde ; } , } ; quote ! { # [doc (hidden)] # [allow (non_upper_case_globals , unused_attributes , unused_qualifications , clippy :: absolute_paths ,)] const _ : () = { # use_serde # code } ; } }
};
}
