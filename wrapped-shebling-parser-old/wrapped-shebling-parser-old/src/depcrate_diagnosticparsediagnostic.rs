// Generated macro for ParseDiagnostic (struct)
macro_rules! Depcrate_diagnosticParseDiagnostic {
() => {
// Module: crate::diagnostic
// Provides: {"ParseDiagnostic"}
// Dependencies: {}
# [derive (Clone , Debug , Error)] # [error ("{kind}")] pub (crate) struct ParseDiagnostic { kind : ParseDiagnosticKind , labels : Vec < LabeledRange > , help : Option < String > , }
};
}
