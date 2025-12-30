// Generated macro for macro_325 (macro)
macro_rules! Depcrate_exprmacro_325 {
() => {
// Module: crate::expr
// Provides: {"macro_325"}
// Dependencies: {}
ast_struct ! { # [doc = " A parenthesized expression: `(a + b)`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprParen { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub expr : Box < Expr >, } }
};
}
