// Generated macro for macro_323 (macro)
macro_rules! Depcrate_exprmacro_323 {
() => {
// Module: crate::expr
// Provides: {"macro_323"}
// Dependencies: {}
ast_struct ! { # [doc = " A `match` expression: `match n { Some(n) => {}, None => {} }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprMatch # full { pub attrs : Vec < Attribute >, pub match_token : Token ! [match] , pub expr : Box < Expr >, pub brace_token : token :: Brace , pub arms : Vec < Arm >, } }
};
}
