// Generated macro for macro_316 (macro)
macro_rules! Depcrate_exprmacro_316 {
() => {
// Module: crate::expr
// Provides: {"macro_316"}
// Dependencies: {}
ast_struct ! { # [doc = " An `if` expression with an optional `else` block: `if expr { ... }"] # [doc = " else { ... }`."] # [doc = ""] # [doc = " The `else` branch expression may only be an `If` or `Block`"] # [doc = " expression, not any of the other types of expression."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprIf # full { pub attrs : Vec < Attribute >, pub if_token : Token ! [if] , pub cond : Box < Expr >, pub then_branch : Block , pub else_branch : Option < (Token ! [else] , Box < Expr >) >, } }
};
}
