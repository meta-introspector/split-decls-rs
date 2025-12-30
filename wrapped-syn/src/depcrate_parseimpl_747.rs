// Generated macro for impl_747 (impl)
macro_rules! Depcrate_parseimpl_747 {
() => {
// Module: crate::parse
// Provides: {"impl_747"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl < T : Parse > Parse for Box < T > { fn parse (input : ParseStream) -> Result < Self > { input . parse () . map (Box :: new) } }
};
}
