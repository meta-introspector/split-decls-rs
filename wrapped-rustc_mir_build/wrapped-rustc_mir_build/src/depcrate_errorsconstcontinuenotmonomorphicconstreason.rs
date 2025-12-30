// Generated macro for ConstContinueNotMonomorphicConstReason (enum)
macro_rules! Depcrate_errorsConstContinueNotMonomorphicConstReason {
() => {
// Module: crate::errors
// Provides: {"ConstContinueNotMonomorphicConstReason"}
// Dependencies: {}
# [derive (Subdiagnostic)] pub (crate) enum ConstContinueNotMonomorphicConstReason { # [label (mir_build_const_continue_not_const_constant_parameter)] ConstantParameter { # [primary_span] span : Span , } , # [label (mir_build_const_continue_not_const_const_block)] ConstBlock { # [primary_span] span : Span , } , # [label (mir_build_const_continue_not_const_const_other)] Other { # [primary_span] span : Span , } , }
};
}
