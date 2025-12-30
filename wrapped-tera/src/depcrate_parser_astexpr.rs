// Generated macro for Expr (struct)
macro_rules! Depcrate_parser_astExpr {
() => {
// Module: crate::parser::ast
// Provides: {"Expr"}
// Dependencies: {}
# [doc = " An expression is a value that can be negated and followed by"] # [doc = " optional filters"] # [derive (Clone , Debug , PartialEq)] pub struct Expr { # [doc = " The expression we are evaluating"] pub val : ExprVal , # [doc = " Is it using `not`?"] pub negated : bool , # [doc = " List of filters used on that value"] pub filters : Vec < FunctionCall > , }
};
}
