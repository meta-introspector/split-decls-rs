// Generated macro for macro_108 (macro)
macro_rules! Depcrate_attrmacro_108 {
() => {
// Module: crate::attr
// Provides: {"macro_108"}
// Dependencies: {}
ast_struct ! { # [doc = " A structured list within an attribute, like `derive(Copy, Clone)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct MetaList { pub path : Path , pub delimiter : MacroDelimiter , pub tokens : TokenStream , } }
};
}
