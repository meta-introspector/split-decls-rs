// Generated macro for parse_single_variant (function)
macro_rules! Depcrateparse_single_variant {
() => {
// Module: crate
// Provides: {"parse_single_variant"}
// Dependencies: {}
# [proc_macro] pub fn parse_single_variant (input : TokenStream) -> TokenStream { match parse2 :: < SingleVariant > (input . into ()) { Ok (_) => quote ! () , Err (e) => e . into_compile_error () , } . into () }
};
}
