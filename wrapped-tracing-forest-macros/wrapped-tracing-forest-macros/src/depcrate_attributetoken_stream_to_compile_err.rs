// Generated macro for token_stream_to_compile_err (function)
macro_rules! Depcrate_attributetoken_stream_to_compile_err {
() => {
// Module: crate::attribute
// Provides: {"token_stream_to_compile_err"}
// Dependencies: {}
fn token_stream_to_compile_err (mut tokens : TokenStream , err : syn :: Error) -> TokenStream { tokens . extend (TokenStream :: from (err . into_compile_error ())) ; tokens }
};
}
