// Generated macro for LogicExpr (struct)
macro_rules! Depcrate_parser_astLogicExpr {
() => {
// Module: crate::parser::ast
// Provides: {"LogicExpr"}
// Dependencies: {}
# [doc = " A logical expression"] # [derive (Clone , Debug , PartialEq)] pub struct LogicExpr { # [doc = " The left hand side of the expression"] pub lhs : Box < Expr > , # [doc = " The right hand side of the expression"] pub rhs : Box < Expr > , # [doc = " The operator used"] pub operator : LogicOperator , }
};
}
