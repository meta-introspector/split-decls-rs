// Generated macro for MovedWhileBorrowed (struct)
macro_rules! Depcrate_errorsMovedWhileBorrowed {
() => {
// Module: crate::errors
// Provides: {"MovedWhileBorrowed"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (mir_build_moved_while_borrowed)] pub (crate) struct MovedWhileBorrowed { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
};
}
