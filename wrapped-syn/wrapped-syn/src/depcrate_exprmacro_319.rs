// Generated macro for macro_319 (macro)
macro_rules! Depcrate_exprmacro_319 {
() => {
// Module: crate::expr
// Provides: {"macro_319"}
// Dependencies: {}
ast_struct ! { # [doc = " A `let` guard: `let Some(x) = opt`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprLet # full { pub attrs : Vec < Attribute >, pub let_token : Token ! [let] , pub pat : Box < Pat >, pub eq_token : Token ! [=] , pub expr : Box < Expr >, } }
};
}
