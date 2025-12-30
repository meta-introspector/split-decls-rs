// Generated macro for impl_732 (impl)
macro_rules! Depcrate_parseimpl_732 {
() => {
// Module: crate::parse
// Provides: {"impl_732"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl < T : Parse + Token > Parse for Option < T > { fn parse (input : ParseStream) -> Result < Self > { if T :: peek (input . cursor ()) { Ok (Some (input . parse () ?)) } else { Ok (None) } } }
};
}
