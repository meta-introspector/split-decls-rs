// Generated macro for impl_8 (impl)
macro_rules! Depcrate_diagnosticimpl_8 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_8"}
// Dependencies: {}
impl DiagnosticBuilder { # [doc = " Adds a labeled span to the [Diagnostic]."] pub fn label (mut self , label : impl AsRef < str > , span : impl Into < miette :: SourceSpan >) -> Self { self . labels . push (miette :: LabeledSpan :: new_with_span (Some (label . as_ref () . into ()) , span . into () ,)) ; self } # [doc = " Adds a labeled span with an empty message to the [Diagnostic]."] pub fn span (mut self , span : impl Into < miette :: SourceSpan >) -> Self { self . labels . push (miette :: LabeledSpan :: underline (span . into ())) ; self } # [doc = " Sets the help message of the [Diagnostic]."] pub fn help (mut self , help : impl AsRef < str >) -> Self { self . help = Some (help . as_ref () . into ()) ; self } # [doc = " Builds the [Diagnostic]."] pub fn build (self) -> Diagnostic { Diagnostic { kind : self . kind , labels : self . labels , help : self . help , } } }
};
}
