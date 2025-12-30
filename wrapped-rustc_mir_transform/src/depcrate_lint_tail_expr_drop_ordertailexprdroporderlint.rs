// Generated macro for TailExprDropOrderLint (struct)
macro_rules! Depcrate_lint_tail_expr_drop_orderTailExprDropOrderLint {
() => {
// Module: crate::lint_tail_expr_drop_order
// Provides: {"TailExprDropOrderLint"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_transform_tail_expr_drop_order)] struct TailExprDropOrderLint < 'a > { # [subdiagnostic] local_labels : Vec < LocalLabel < 'a > > , # [label (mir_transform_drop_location)] drop_span : Option < Span > , # [note (mir_transform_note_epilogue)] _epilogue : () , }
};
}
