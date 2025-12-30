// Generated macro for impl_84 (impl)
macro_rules! Depcrate_diagnosticimpl_84 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_84"}
// Dependencies: {}
impl ParseDiagnostic { # [doc = " Creates a new [builder](ParseDiagnosticBuilder) for a diagnostic"] # [doc = " of the given [kind](ParseDiagnosticKind)."] pub (crate) fn builder (kind : ParseDiagnosticKind) -> ParseDiagnosticBuilder { let help = (& kind as & dyn miette :: Diagnostic) . help () . map (| help | help . to_string ()) ; ParseDiagnosticBuilder { kind , labels : vec ! [] , help , } } # [doc = " The [Range] of the first label, which is used as the diagnostic's"] # [doc = " main range."] pub (crate) fn range (& self) -> & Range { & self . labels . first () . expect ("Diagnostics should have at least one label.") . 1 } }
};
}
