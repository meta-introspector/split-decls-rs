// Generated macro for derive_parse (function)
macro_rules! Depcratederive_parse {
() => {
// Module: crate
// Provides: {"derive_parse"}
// Dependencies: {}
# [proc_macro_derive (Parse , attributes (to_tokens , parse))] pub fn derive_parse (input : proc_macro :: TokenStream) -> proc_macro :: TokenStream { let input = parse_macro_input ! (input as DeriveInput) ; into_macro_output (parse :: derive_parse (input)) }
};
}
