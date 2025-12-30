// Generated macro for AlreadyMutBorrowed (struct)
macro_rules! Depcrate_errorsAlreadyMutBorrowed {
() => {
// Module: crate::errors
// Provides: {"AlreadyMutBorrowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_already_mut_borrowed)] pub (crate) struct AlreadyMutBorrowed { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
};
}
