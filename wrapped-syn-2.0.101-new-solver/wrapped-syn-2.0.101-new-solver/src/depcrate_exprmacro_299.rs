// Generated macro for macro_299 (macro)
macro_rules! Depcrate_exprmacro_299 {
() => {
// Module: crate::expr
// Provides: {"macro_299"}
// Dependencies: {}
ast_struct ! { # [doc = " A slice literal expression: `[a, b, c, d]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprArray # full { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub elems : Punctuated < Expr , Token ! [,] >, } }
};
}
