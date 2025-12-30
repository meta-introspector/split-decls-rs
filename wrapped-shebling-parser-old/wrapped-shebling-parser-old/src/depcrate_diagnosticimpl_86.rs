// Generated macro for impl_86 (impl)
macro_rules! Depcrate_diagnosticimpl_86 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_86"}
// Dependencies: {}
impl ParseDiagnosticBuilder { # [doc = " Adds a [LabeledRange] to the [ParseDiagnostic]."] pub (crate) fn label (mut self , label : impl AsRef < str > , range : impl Into < Range >) -> Self { self . labels . push ((label . as_ref () . into () , range . into ())) ; self } # [doc = " Adds a [LabeledRange] with an empty message to the [ParseDiagnostic]."] pub (crate) fn range (mut self , range : impl Into < Range >) -> Self { self . labels . push (("" . into () , range . into ())) ; self } # [doc = " Sets the help message of the [ParseDiagnostic]."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if the diagnostic already has a help message."] pub (crate) fn help (mut self , help : impl AsRef < str >) -> Self { if self . help . is_some () { panic ! ("This diagnostic already has a help message.") ; } self . help = Some (help . as_ref () . into ()) ; self } # [doc = " Builds the [ParseDiagnostic]."] pub (super) fn build (self) -> ParseDiagnostic { ParseDiagnostic { kind : self . kind , labels : self . labels , help : self . help , } } }
};
}
