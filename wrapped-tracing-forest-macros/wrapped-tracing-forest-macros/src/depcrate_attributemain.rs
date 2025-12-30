// Generated macro for main (function)
macro_rules! Depcrate_attributemain {
() => {
// Module: crate::attribute
// Provides: {"main"}
// Dependencies: {}
pub fn main (args : TokenStream , item : TokenStream) -> TokenStream { let input : syn :: ItemFn = match syn :: parse (item . clone ()) { Ok (input) => input , Err (e) => return token_stream_to_compile_err (item , e) , } ; impl_attribute (input , args , false) . unwrap_or_else (| e | token_stream_to_compile_err (item , e)) }
};
}
