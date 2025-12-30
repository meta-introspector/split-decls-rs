// Generated macro for impl_733 (impl)
macro_rules! Depcrate_parseimpl_733 {
() => {
// Module: crate::parse
// Provides: {"impl_733"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for TokenStream { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | Ok ((cursor . token_stream () , Cursor :: empty ()))) } }
};
}
