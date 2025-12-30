// Generated macro for to_proptest_config (function)
macro_rules! Depcrate_proptest_fnto_proptest_config {
() => {
// Module: crate::proptest_fn
// Provides: {"to_proptest_config"}
// Dependencies: {}
fn to_proptest_config (args : Args) -> TokenStream { if args . is_empty () { return quote ! () ; } let mut base_expr = None ; let mut inits = Vec :: new () ; for arg in args { match arg { Arg :: Value (value) => base_expr = Some (value) , Arg :: NameValue { name , value , .. } => inits . push (quote ! (# name : # value)) , } } let base_expr = base_expr . unwrap_or_else (| | { parse_quote ! (< proptest :: test_runner :: Config as std :: default :: Default >:: default ()) }) ; quote ! { #! [proptest_config (proptest :: test_runner :: Config { # (# inits ,) * .. # base_expr })] } }
};
}
