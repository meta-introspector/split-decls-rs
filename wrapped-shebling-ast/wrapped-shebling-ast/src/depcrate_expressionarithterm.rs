// Generated macro for ArithTerm (enum)
macro_rules! Depcrate_expressionArithTerm {
() => {
// Module: crate::expression
// Provides: {"ArithTerm"}
// Dependencies: {}
# [doc = " Arithmetic expressions."] # [derive (Debug , From)] # [decl (enum , name = "ArithTerm" , vis = "pub" , hash = "97677bce")] pub enum ArithTerm { # [doc = " Binary expression (e.g. `$x + 1`)."] BinExpr (BinExpr < Spanned < BinOp > , ArithTerm >) , # [doc = " Shell expansion."] Expansion (Vec < WordSgmt >) , # [doc = " Parenthesized expression."] Group (ArithSeq) , # [doc = " Conditional expression with a ternary operator."] TriExpr (ArithTriExpr) , # [doc = " Unary expression (e.g. `++x`)."] UnExpr (UnExpr < Spanned < UnOp > , ArithTerm >) , # [doc = " A (maybe indexed) variable."] Var (SubscriptedVar) , }
};
}
