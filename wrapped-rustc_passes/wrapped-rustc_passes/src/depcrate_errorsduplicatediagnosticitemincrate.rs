// Generated macro for DuplicateDiagnosticItemInCrate (struct)
macro_rules! Depcrate_errorsDuplicateDiagnosticItemInCrate {
() => {
// Module: crate::errors
// Provides: {"DuplicateDiagnosticItemInCrate"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (passes_duplicate_diagnostic_item_in_crate)] pub (crate) struct DuplicateDiagnosticItemInCrate { # [primary_span] pub duplicate_span : Option < Span > , # [note (passes_diagnostic_item_first_defined)] pub orig_span : Option < Span > , # [note] pub different_crates : bool , pub crate_name : Symbol , pub orig_crate_name : Symbol , pub name : Symbol , }
};
}
