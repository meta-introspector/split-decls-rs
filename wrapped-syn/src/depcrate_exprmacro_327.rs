// Generated macro for macro_327 (macro)
macro_rules! Depcrate_exprmacro_327 {
() => {
// Module: crate::expr
// Provides: {"macro_327"}
// Dependencies: {}
ast_struct ! { # [doc = " A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRange # full { pub attrs : Vec < Attribute >, pub start : Option < Box < Expr >>, pub limits : RangeLimits , pub end : Option < Box < Expr >>, } }
};
}
