// Generated macro for impl_734 (impl)
macro_rules! Depcrate_parseimpl_734 {
() => {
// Module: crate::parse
// Provides: {"impl_734"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for TokenTree { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . token_tree () { Some ((tt , rest)) => Ok ((tt , rest)) , None => Err (cursor . error ("expected token tree")) , }) } }
};
}
