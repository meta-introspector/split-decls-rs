// Generated macro for impl_async (function)
macro_rules! Depcrate_attributeimpl_async {
() => {
// Module: crate::attribute
// Provides: {"impl_async"}
// Dependencies: {}
# [cfg (feature = "sync")] fn impl_async (config : Config , mut input : syn :: ItemFn) -> syn :: Result < TokenStream > { let builder = config . builder () ; let brace_token = input . block . brace_token ; let block = input . block ; input . block = syn :: parse2 (quote ! { { # builder . async_layer () . on_future (async # block) . await } }) . expect ("Parsing failure") ; input . block . brace_token = brace_token ; Ok (quote ! { # input } . into ()) }
};
}
