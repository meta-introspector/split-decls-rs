// Generated macro for impl_macros (macro)
macro_rules! Depcrateimpl_macros {
() => {
// Module: crate
// Provides: {"impl_macros"}
// Dependencies: {}
macro_rules ! impl_macros { ($ ($ name : ident) *) => { $ (# [proc_macro] pub fn $ name (input : TokenStream) -> TokenStream { use crate :: to_tokens :: ToTokenStream ; let mut iter = input . into_iter () . peekable () ; match $ name :: parse (& mut iter) { Ok (value) => match iter . peek () { Some (tree) => Error :: UnexpectedToken { tree : tree . clone () } . to_compile_error () , None => quote_ ! { const { # S (value . into_token_stream ()) } } , } , Err (err) => err . to_compile_error () , } }) * } ; }
};
}
