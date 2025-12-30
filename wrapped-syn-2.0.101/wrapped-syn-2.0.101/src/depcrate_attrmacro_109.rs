// Generated macro for macro_109 (macro)
macro_rules! Depcrate_attrmacro_109 {
() => {
// Module: crate::attr
// Provides: {"macro_109"}
// Dependencies: {}
ast_struct ! { # [doc = " A name-value pair within an attribute, like `feature = \"nightly\"`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct MetaNameValue { pub path : Path , pub eq_token : Token ! [=] , pub value : Expr , } }
};
}
