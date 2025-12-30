// Generated macro for impl_731 (impl)
macro_rules! Depcrate_parseimpl_731 {
() => {
// Module: crate::parse
// Provides: {"impl_731"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl < T : Parse > Parse for Box < T > { fn parse (input : ParseStream) -> Result < Self > { input . parse () . map (Box :: new) } }
};
}
