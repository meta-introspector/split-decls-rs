// Generated macro for derive_arbitrary (function)
macro_rules! Depcratederive_arbitrary {
() => {
// Module: crate
// Provides: {"derive_arbitrary"}
// Dependencies: {}
# [proc_macro_derive (Arbitrary , attributes (arbitrary , strategy , any , map , filter , weight , by_ref))] pub fn derive_arbitrary (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; into_macro_output (arbitrary :: derive_arbitrary (input)) }
};
}
