// Generated macro for DiagnosticCode (struct)
macro_rules! Depcrate_diagnosticsDiagnosticCode {
() => {
// Module: crate::diagnostics
// Provides: {"DiagnosticCode"}
// Dependencies: {}
# [doc = " The error code emitted by the compiler. See [Rust error codes index]."] # [doc = ""] # [doc = " [Rust error codes index]: https://doc.rust-lang.org/error_codes/error-index.html"] # [derive (Clone , Deserialize , Debug , Eq , PartialEq , Hash)] pub struct DiagnosticCode { # [doc = " The code itself."] pub code : String , # [doc = " An explanation for the code."] explanation : Option < String > , }
};
}
