// Generated macro for ArithTriExpr (struct)
macro_rules! Depcrate_expressionArithTriExpr {
() => {
// Module: crate::expression
// Provides: {"ArithTriExpr"}
// Dependencies: {}
# [doc = " An arithmetic ternary expression of the form"] # [doc = " `condition ? true_branch : else_branch`."] # [derive (Debug)] # [decl (struct , name = "ArithTriExpr" , vis = "pub" , hash = "a6c1c893")] pub struct ArithTriExpr { cond : Box < ArithTerm > , then_branch : Box < ArithTerm > , else_branch : Box < ArithTerm > , }
};
}
