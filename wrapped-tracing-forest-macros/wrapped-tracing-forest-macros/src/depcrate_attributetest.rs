// Generated macro for test (function)
macro_rules! Depcrate_attributetest {
() => {
// Module: crate::attribute
// Provides: {"test"}
// Dependencies: {}
pub fn test (args : TokenStream , item : TokenStream) -> TokenStream { let input : syn :: ItemFn = match syn :: parse (item . clone ()) { Ok (input) => input , Err (e) => return token_stream_to_compile_err (item , e) , } ; if let Some (attr) = input . attrs . iter () . find (| attr | attr . path . is_ident ("test")) { let msg = "Second #[test] attribute is supplied" ; return token_stream_to_compile_err (item , syn :: Error :: new_spanned (& attr , msg)) ; } impl_attribute (input , args , true) . unwrap_or_else (| e | token_stream_to_compile_err (item , e)) }
};
}
