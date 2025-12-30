// Generated macro for macro_305 (macro)
macro_rules! Depcrate_exprmacro_305 {
() => {
// Module: crate::expr
// Provides: {"macro_305"}
// Dependencies: {}
ast_struct ! { # [doc = " A `break`, with an optional label to break and an optional"] # [doc = " expression."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprBreak # full { pub attrs : Vec < Attribute >, pub break_token : Token ! [break] , pub label : Option < Lifetime >, pub expr : Option < Box < Expr >>, } }
};
}
