// Generated macro for Cycle (struct)
macro_rules! Depcrate_errorCycle {
() => {
// Module: crate::error
// Provides: {"Cycle"}
// Dependencies: {}
# [derive (Diagnostic)] # [diag (query_system_cycle , code = E0391)] pub (crate) struct Cycle { # [primary_span] pub span : Span , pub stack_bottom : String , # [subdiagnostic] pub cycle_stack : Vec < CycleStack > , # [subdiagnostic] pub stack_count : StackCount , # [subdiagnostic] pub alias : Option < Alias > , # [subdiagnostic] pub cycle_usage : Option < CycleUsage > , # [note] pub note_span : () , }
};
}
