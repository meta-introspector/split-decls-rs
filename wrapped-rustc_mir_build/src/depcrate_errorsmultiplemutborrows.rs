// Generated macro for MultipleMutBorrows (struct)
macro_rules! Depcrate_errorsMultipleMutBorrows {
() => {
// Module: crate::errors
// Provides: {"MultipleMutBorrows"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_multiple_mut_borrows)] pub (crate) struct MultipleMutBorrows { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
};
}
