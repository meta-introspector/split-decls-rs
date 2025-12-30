// Generated macro for ArithTerm (enum)
macro_rules! Depcrate_astArithTerm {
() => {
// Module: crate::ast
// Provides: {"ArithTerm"}
// Dependencies: {}
# [derive (Debug , From , PartialEq)] pub (crate) enum ArithTerm { # [doc = " Binary expression (e.g. `$x + 1`)."] BinExpr (BinExpr < BinOp , ArithTerm >) , # [doc = " Shell expansion."] Expansion (Vec < WordSgmt >) , # [doc = " Parenthesized expression."] Group (ArithSeq) , # [doc = " Conditional expression with a ternary operator."] TriExpr (ArithTriExpr) , # [doc = " Unary expression (e.g. `++x`)."] UnExpr (UnExpr < UnOp , ArithTerm >) , # [doc = " A (maybe indexed) variable."] Var (SubscriptedVar) , }
};
}
