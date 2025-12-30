// Generated macro for declare_id (function)
macro_rules! Depcratedeclare_id {
() => {
// Module: crate
// Provides: {"declare_id"}
// Dependencies: {}
# [proc_macro] pub fn declare_id (input : TokenStream) -> TokenStream { let id = parse_macro_input ! (input as Id) ; TokenStream :: from (quote ! { # id }) }
};
}
