// Generated macro for CondExpr (enum)
macro_rules! Depcrate_astCondExpr {
() => {
// Module: crate::ast
// Provides: {"CondExpr"}
// Dependencies: {}
# [derive (Debug , From , PartialEq)] pub (crate) enum CondExpr { # [doc = " A binary expression (e.g. `x -eq y`)."] BinExpr (BinExpr < CondOp , CondExpr >) , # [doc = " Parenthesized expression (e.g. `( x -eq y )`)."] Group (Box < CondExpr >) , # [doc = " A binary expression (e.g. `-z x`)."] UnExpr (UnExpr < CondOp , CondExpr >) , # [doc = " A nullary expression."] Word (Word) , }
};
}
