// Generated macro for macro_306 (macro)
macro_rules! Depcrate_exprmacro_306 {
() => {
// Module: crate::expr
// Provides: {"macro_306"}
// Dependencies: {}
ast_struct ! { # [doc = " A function call expression: `invoke(a, b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprCall { pub attrs : Vec < Attribute >, pub func : Box < Expr >, pub paren_token : token :: Paren , pub args : Punctuated < Expr , Token ! [,] >, } }
};
}
