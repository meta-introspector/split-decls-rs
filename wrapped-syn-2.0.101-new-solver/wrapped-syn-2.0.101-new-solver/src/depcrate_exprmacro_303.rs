// Generated macro for macro_303 (macro)
macro_rules! Depcrate_exprmacro_303 {
() => {
// Module: crate::expr
// Provides: {"macro_303"}
// Dependencies: {}
ast_struct ! { # [doc = " A binary operation: `a + b`, `a += b`."] # [cfg_attr (docsrs , doc (cfg (any (feature = "full" , feature = "derive"))))] pub struct ExprBinary { pub attrs : Vec < Attribute >, pub left : Box < Expr >, pub op : BinOp , pub right : Box < Expr >, } }
};
}
