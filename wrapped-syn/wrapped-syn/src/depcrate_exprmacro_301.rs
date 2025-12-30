// Generated macro for macro_301 (macro)
macro_rules! Depcrate_exprmacro_301 {
() => {
// Module: crate::expr
// Provides: {"macro_301"}
// Dependencies: {}
ast_struct ! { # [doc = " A slice literal expression: `[a, b, c, d]`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprArray # full { pub attrs : Vec < Attribute >, pub bracket_token : token :: Bracket , pub elems : Punctuated < Expr , Token ! [,] >, } }
};
}
