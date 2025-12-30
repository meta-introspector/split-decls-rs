// Generated macro for parse_attr (function)
macro_rules! Depcrateparse_attr {
() => {
// Module: crate
// Provides: {"parse_attr"}
// Dependencies: {}
fn parse_attr < T : syn :: parse :: Parse > (attr : TokenStream , item : TokenStream) -> TokenStream { match parse :: < T > (attr) { Ok (_) => item , Err (e) => e . into_compile_error () . into () , } }
};
}
