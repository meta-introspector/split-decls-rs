// Generated macro for impl_19 (impl)
macro_rules! Depcrate_expressionimpl_19 {
() => {
// Module: crate::expression
// Provides: {"impl_19"}
// Dependencies: {}
impl ArithTriExpr { # [doc = " Creates a new arithmetic ternary expression."] pub fn new (cond : impl Into < ArithTerm > , then_branch : impl Into < ArithTerm > , else_branch : impl Into < ArithTerm > ,) -> Self { Self { cond : Box :: new (cond . into ()) , then_branch : Box :: new (then_branch . into ()) , else_branch : Box :: new (else_branch . into ()) , } } }
};
}
