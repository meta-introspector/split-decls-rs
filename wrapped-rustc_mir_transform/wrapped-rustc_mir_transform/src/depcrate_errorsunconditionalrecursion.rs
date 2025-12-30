// Generated macro for UnconditionalRecursion (struct)
macro_rules! Depcrate_errorsUnconditionalRecursion {
() => {
// Module: crate::errors
// Provides: {"UnconditionalRecursion"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (mir_transform_unconditional_recursion)] # [help] pub (crate) struct UnconditionalRecursion { # [label] pub (crate) span : Span , # [label (mir_transform_unconditional_recursion_call_site_label)] pub (crate) call_sites : Vec < Span > , }
};
}
