// Generated macro for impl_736 (impl)
macro_rules! Depcrate_parseimpl_736 {
() => {
// Module: crate::parse
// Provides: {"impl_736"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Punct { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | match cursor . punct () { Some ((punct , rest)) => Ok ((punct , rest)) , None => Err (cursor . error ("expected punctuation token")) , }) } }
};
}
