// Generated macro for ArithTriExpr (struct)
macro_rules! Depcrate_astArithTriExpr {
() => {
// Module: crate::ast
// Provides: {"ArithTriExpr"}
// Dependencies: {}
# [doc = " An arithmetic ternary expression of the form"] # [doc = " `condition ? true_branch : else_branch`."] # [derive (Debug , PartialEq)] pub (crate) struct ArithTriExpr { cond : Box < ArithTerm > , then_branch : Box < ArithTerm > , else_branch : Box < ArithTerm > , }
};
}
