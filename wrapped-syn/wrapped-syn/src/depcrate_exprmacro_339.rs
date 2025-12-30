// Generated macro for macro_339 (macro)
macro_rules! Depcrate_exprmacro_339 {
() => {
// Module: crate::expr
// Provides: {"macro_339"}
// Dependencies: {}
ast_struct ! { # [doc = " A yield expression: `yield expr`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprYield # full { pub attrs : Vec < Attribute >, pub yield_token : Token ! [yield] , pub expr : Option < Box < Expr >>, } }
};
}
