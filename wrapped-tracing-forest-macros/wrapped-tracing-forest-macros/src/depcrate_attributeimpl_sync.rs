// Generated macro for impl_sync (function)
macro_rules! Depcrate_attributeimpl_sync {
() => {
// Module: crate::attribute
// Provides: {"impl_sync"}
// Dependencies: {}
fn impl_sync (config : Config , mut input : syn :: ItemFn) -> syn :: Result < TokenStream > { let header = if config . is_test { quote ! { # [:: core :: prelude :: v1 :: test] } } else { quote ! { } } ; let builder = config . builder () ; let brace_token = input . block . brace_token ; let block = input . block ; input . block = syn :: parse2 (quote ! { { # builder . blocking_layer () . on_closure (|| # block) } }) . expect ("Parsing failure") ; input . block . brace_token = brace_token ; Ok (quote ! { # header # input } . into ()) }
};
}
