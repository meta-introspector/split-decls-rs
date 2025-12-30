// Generated macro for test (function)
macro_rules! Depcrate_entrytest {
() => {
// Module: crate::entry
// Provides: {"test"}
// Dependencies: {}
pub (crate) fn test (args : TokenStream , item : TokenStream , rt_multi_thread : bool) -> TokenStream { let input : ItemFn = match syn :: parse2 (item . clone ()) { Ok (it) => it , Err (e) => return token_stream_with_error (item , e) , } ; let config = if let Some (attr) = input . attrs () . find (| attr | is_test_attribute (attr)) { let msg = "second test attribute is supplied, consider removing or changing the order of your test attributes" ; Err (syn :: Error :: new_spanned (attr , msg)) } else { AttributeArgs :: parse_terminated . parse2 (args) . and_then (| args | build_config (& input , args , true , rt_multi_thread)) } ; match config { Ok (config) => parse_knobs (input , true , config) , Err (e) => token_stream_with_error (parse_knobs (input , true , DEFAULT_ERROR_CONFIG) , e) , } }
};
}
