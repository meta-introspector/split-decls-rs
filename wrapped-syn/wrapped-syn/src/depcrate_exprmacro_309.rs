// Generated macro for macro_309 (macro)
macro_rules! Depcrate_exprmacro_309 {
() => {
// Module: crate::expr
// Provides: {"macro_309"}
// Dependencies: {}
ast_struct ! { # [doc = " A cast expression: `foo as f64`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprCast { pub attrs : Vec < Attribute >, pub expr : Box < Expr >, pub as_token : Token ! [as] , pub ty : Box < Type >, } }
};
}
