// Generated macro for macro_333 (macro)
macro_rules! Depcrate_exprmacro_333 {
() => {
// Module: crate::expr
// Provides: {"macro_333"}
// Dependencies: {}
ast_struct ! { # [doc = " A tuple expression: `(a, b, c, d)`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprTuple { pub attrs : Vec < Attribute >, pub paren_token : token :: Paren , pub elems : Punctuated < Expr , Token ! [,] >, } }
};
}
