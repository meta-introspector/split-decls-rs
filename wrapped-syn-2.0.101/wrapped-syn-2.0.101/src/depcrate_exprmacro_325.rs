// Generated macro for macro_325 (macro)
macro_rules! Depcrate_exprmacro_325 {
() => {
// Module: crate::expr
// Provides: {"macro_325"}
// Dependencies: {}
ast_struct ! { # [doc = " A range expression: `1..2`, `1..`, `..2`, `1..=2`, `..=2`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprRange # full { pub attrs : Vec < Attribute >, pub start : Option < Box < Expr >>, pub limits : RangeLimits , pub end : Option < Box < Expr >>, } }
};
}
