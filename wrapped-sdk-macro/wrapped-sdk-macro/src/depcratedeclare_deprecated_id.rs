// Generated macro for declare_deprecated_id (function)
macro_rules! Depcratedeclare_deprecated_id {
() => {
// Module: crate
// Provides: {"declare_deprecated_id"}
// Dependencies: {}
# [proc_macro] pub fn declare_deprecated_id (input : TokenStream) -> TokenStream { let id = parse_macro_input ! (input as IdDeprecated) ; TokenStream :: from (quote ! { # id }) }
};
}
