// Generated macro for macro_329 (macro)
macro_rules! Depcrate_exprmacro_329 {
() => {
// Module: crate::expr
// Provides: {"macro_329"}
// Dependencies: {}
ast_struct ! { # [doc = " A `return`, with an optional value to be returned."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprReturn # full { pub attrs : Vec < Attribute >, pub return_token : Token ! [return] , pub expr : Option < Box < Expr >>, } }
};
}
