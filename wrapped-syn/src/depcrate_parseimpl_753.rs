// Generated macro for impl_753 (impl)
macro_rules! Depcrate_parseimpl_753 {
() => {
// Module: crate::parse
// Provides: {"impl_753"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Literal { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . literal () { Some ((literal , rest)) => Ok ((literal , rest)) , None => Err (cursor . error ("expected literal token")) , }) } }
};
}
