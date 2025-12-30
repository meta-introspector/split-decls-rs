// Generated macro for main (function)
macro_rules! Depcrate_entrymain {
() => {
// Module: crate::entry
// Provides: {"main"}
// Dependencies: {}
pub (crate) fn main (args : TokenStream , item : TokenStream , rt_multi_thread : bool) -> TokenStream { let input : ItemFn = match syn :: parse2 (item . clone ()) { Ok (it) => it , Err (e) => return token_stream_with_error (item , e) , } ; let config = if input . sig . ident == "main" && ! input . sig . inputs . is_empty () { let msg = "the main function cannot accept arguments" ; Err (syn :: Error :: new_spanned (& input . sig . ident , msg)) } else { AttributeArgs :: parse_terminated . parse2 (args) . and_then (| args | build_config (& input , args , false , rt_multi_thread)) } ; match config { Ok (config) => parse_knobs (input , false , config) , Err (e) => token_stream_with_error (parse_knobs (input , false , DEFAULT_ERROR_CONFIG) , e) , } }
};
}
