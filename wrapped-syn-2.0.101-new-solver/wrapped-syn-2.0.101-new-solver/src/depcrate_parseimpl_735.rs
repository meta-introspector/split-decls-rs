// Generated macro for impl_735 (impl)
macro_rules! Depcrate_parseimpl_735 {
() => {
// Module: crate::parse
// Provides: {"impl_735"}
// Dependencies: {}
# [cfg_attr (docsrs , doc (cfg (feature = "parsing")))] impl Parse for Group { fn parse (input : ParseStream) -> Result < Self > { input . step (| cursor | { if let Some ((group , rest)) = cursor . any_group_token () { if group . delimiter () != Delimiter :: None { return Ok ((group , rest)) ; } } Err (cursor . error ("expected group token")) }) } }
};
}
