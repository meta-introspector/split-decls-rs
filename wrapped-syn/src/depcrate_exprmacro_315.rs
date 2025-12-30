// Generated macro for macro_315 (macro)
macro_rules! Depcrate_exprmacro_315 {
() => {
// Module: crate::expr
// Provides: {"macro_315"}
// Dependencies: {}
ast_struct ! { # [doc = " An expression contained within invisible delimiters."] # [doc = ""] # [doc = " This variant is important for faithfully representing the precedence"] # [doc = " of expressions and is related to `None`-delimited spans in a"] # [doc = " `TokenStream`."] # [cfg_attr (docsrs , doc (cfg (feature = "full")))] pub struct ExprGroup { pub attrs : Vec < Attribute >, pub group_token : token :: Group , pub expr : Box < Expr >, } }
};
}
