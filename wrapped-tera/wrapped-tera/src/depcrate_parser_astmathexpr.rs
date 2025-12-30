// Generated macro for MathExpr (struct)
macro_rules! Depcrate_parser_astMathExpr {
() => {
// Module: crate::parser::ast
// Provides: {"MathExpr"}
// Dependencies: {}
# [doc = " A mathematical expression"] # [derive (Clone , Debug , PartialEq)] pub struct MathExpr { # [doc = " The left hand side of the expression"] pub lhs : Box < Expr > , # [doc = " The right hand side of the expression"] pub rhs : Box < Expr > , # [doc = " The operator used"] pub operator : MathOperator , }
};
}
