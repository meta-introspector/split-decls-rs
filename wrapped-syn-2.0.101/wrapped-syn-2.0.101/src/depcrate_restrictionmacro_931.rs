// Generated macro for macro_931 (macro)
macro_rules! Depcrate_restrictionmacro_931 {
() => {
// Module: crate::restriction
// Provides: {"macro_931"}
// Dependencies: {}
ast_struct ! { # [doc = " A visibility level restricted to some path: `pub(self)` or"] # [doc = " `pub(super)` or `pub(crate)` or `pub(in some::module)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct VisRestricted { pub pub_token : Token ! [pub] , pub paren_token : token :: Paren , pub in_token : Option < Token ! [in] >, pub path : Box < Path >, } }
};
}
