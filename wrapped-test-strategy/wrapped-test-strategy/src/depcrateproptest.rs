// Generated macro for proptest (function)
macro_rules! Depcrateproptest {
() => {
// Module: crate
// Provides: {"proptest"}
// Dependencies: {}
# [proc_macro_attribute] pub fn proptest (attr : proc_macro :: TokenStream , item : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let item_fn = parse_macro_input ! (item as ItemFn) ; into_macro_output (proptest_fn :: build_proptest (attr . into () , item_fn)) }
};
}
