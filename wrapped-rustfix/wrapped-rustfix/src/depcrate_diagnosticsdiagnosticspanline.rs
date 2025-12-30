// Generated macro for DiagnosticSpanLine (struct)
macro_rules! Depcrate_diagnosticsDiagnosticSpanLine {
() => {
// Module: crate::diagnostics
// Provides: {"DiagnosticSpanLine"}
// Dependencies: {}
# [doc = " Span information of a single line."] # [derive (Clone , Deserialize , Debug , Eq , PartialEq , Hash)] pub struct DiagnosticSpanLine { pub text : String , # [doc = " 1-based, character offset in self.text."] pub highlight_start : usize , pub highlight_end : usize , }
};
}
