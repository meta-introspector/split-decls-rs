// Generated macro for token_stream_with_error (function)
macro_rules! Depcrate_entrytoken_stream_with_error {
() => {
// Module: crate::entry
// Provides: {"token_stream_with_error"}
// Dependencies: {}
fn token_stream_with_error (mut tokens : TokenStream , error : syn :: Error) -> TokenStream { tokens . extend (error . into_compile_error ()) ; tokens }
};
}
