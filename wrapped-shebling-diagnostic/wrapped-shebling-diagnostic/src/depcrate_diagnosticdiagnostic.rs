// Generated macro for Diagnostic (struct)
macro_rules! Depcrate_diagnosticDiagnostic {
() => {
// Module: crate::diagnostic
// Provides: {"Diagnostic"}
// Dependencies: {}
# [doc = " A `shebling` diagnostic."] # [derive (Debug , thiserror :: Error)] # [error ("{kind}")] # [decl (struct , name = "Diagnostic" , vis = "pub" , hash = "896681ce")] pub struct Diagnostic { kind : DiagnosticKind , labels : Vec < miette :: LabeledSpan > , help : Option < String > , }
};
}
