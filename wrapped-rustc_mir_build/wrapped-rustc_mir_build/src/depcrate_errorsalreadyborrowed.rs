// Generated macro for AlreadyBorrowed (struct)
macro_rules! Depcrate_errorsAlreadyBorrowed {
() => {
// Module: crate::errors
// Provides: {"AlreadyBorrowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_already_borrowed)] pub (crate) struct AlreadyBorrowed { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
};
}
