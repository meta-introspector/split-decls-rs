// Generated macro for derive_to_tokens (function)
macro_rules! Depcratederive_to_tokens {
() => {
// Module: crate
// Provides: {"derive_to_tokens"}
// Dependencies: {}
# [proc_macro_derive (ToTokens , attributes (to_tokens))] pub fn derive_to_tokens (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; into_macro_output (to_tokens :: derive_to_tokens (input)) }
};
}
