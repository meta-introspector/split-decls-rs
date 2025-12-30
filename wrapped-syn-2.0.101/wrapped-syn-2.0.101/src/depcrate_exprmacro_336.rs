// Generated macro for macro_336 (macro)
macro_rules! Depcrate_exprmacro_336 {
() => {
// Module: crate::expr
// Provides: {"macro_336"}
// Dependencies: {}
ast_struct ! { # [doc = " A while loop: `while expr { ... }`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprWhile # full { pub attrs : Vec < Attribute >, pub label : Option < Label >, pub while_token : Token ! [while] , pub cond : Box < Expr >, pub body : Block , } }
};
}
