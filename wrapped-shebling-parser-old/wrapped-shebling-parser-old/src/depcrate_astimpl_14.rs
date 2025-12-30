// Generated macro for impl_14 (impl)
macro_rules! Depcrate_astimpl_14 {
() => {
// Module: crate::ast
// Provides: {"impl_14"}
// Dependencies: {}
impl ArithTriExpr { # [doc = " Creates a new arithmetic ternary expression."] pub (crate) fn new (cond : impl Into < ArithTerm > , then_branch : impl Into < ArithTerm > , else_branch : impl Into < ArithTerm > ,) -> Self { Self { cond : Box :: new (cond . into ()) , then_branch : Box :: new (then_branch . into ()) , else_branch : Box :: new (else_branch . into ()) , } } }
};
}
