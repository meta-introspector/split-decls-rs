// Generated macro for macro_308 (macro)
macro_rules! Depcrate_exprmacro_308 {
() => {
// Module: crate::expr
// Provides: {"macro_308"}
// Dependencies: {}
ast_struct ! { # [doc = " A function call expression: `invoke(a, b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprCall { pub attrs : Vec < Attribute >, pub func : Box < Expr >, pub paren_token : token :: Paren , pub args : Punctuated < Expr , Token ! [,] >, } }
};
}
