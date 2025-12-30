// Generated macro for impl_749 (impl)
macro_rules! Depcrate_parseimpl_749 {
() => {
// Module: crate::parse
// Provides: {"impl_749"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for TokenStream { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | Ok ((cursor . token_stream () , Cursor :: empty ()))) } }
};
}
