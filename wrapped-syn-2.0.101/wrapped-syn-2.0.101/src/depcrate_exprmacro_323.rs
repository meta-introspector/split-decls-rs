// Generated macro for macro_323 (macro)
macro_rules! Depcrate_exprmacro_323 {
() => {
// Module: crate::expr
// Provides: {"macro_323"}
// Dependencies: {}
ast_struct ! { # [doc = " A parenthesized expression: `(a + b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprParen { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub expr : Box < Expr >, } }
};
}
