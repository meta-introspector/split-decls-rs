// Generated macro for macro_1051 (macro)
macro_rules! Depcrate_tymacro_1051 {
() => {
// Module: crate::ty
// Provides: {"macro_1051"}
// Dependencies: {}
ast_enum ! { # [doc = " Return type of a function signature."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub enum ReturnType { # [doc = " Return type is not specified."] # [doc = ""] # [doc = " Functions default to `()` and closures default to type inference."] Default , # [doc = " A particular type is returned."] Type (Token ! [->] , Box < Type >) , } }
};
}
