// Generated macro for pubkeys (function)
macro_rules! Depcratepubkeys {
() => {
// Module: crate
// Provides: {"pubkeys"}
// Dependencies: {}
# [proc_macro] pub fn pubkeys (input : TokenStream) -> TokenStream { let pubkeys = parse_macro_input ! (input as Pubkeys) ; TokenStream :: from (quote ! { # pubkeys }) }
};
}
