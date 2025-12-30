// Generated macro for macro_335 (macro)
macro_rules! Depcrate_exprmacro_335 {
() => {
// Module: crate::expr
// Provides: {"macro_335"}
// Dependencies: {}
ast_struct ! { # [doc = " A tuple expression: `(a, b, c, d)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprTuple { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub elems : Punctuated < Expr , Token ! [,] >, } }
};
}
