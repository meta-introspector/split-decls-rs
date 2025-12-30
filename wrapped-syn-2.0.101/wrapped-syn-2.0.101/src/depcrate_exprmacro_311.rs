// Generated macro for macro_311 (macro)
macro_rules! Depcrate_exprmacro_311 {
() => {
// Module: crate::expr
// Provides: {"macro_311"}
// Dependencies: {}
ast_struct ! { # [doc = " Access of a named struct field (`obj.k`) or unnamed tuple struct"] # [doc = " field (`obj.0`)."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprField { pub attrs : Vec < Attribute >, pub base : Box < Expr >, pub dot_token : Token ! [.] , pub member : Member , } }
};
}
