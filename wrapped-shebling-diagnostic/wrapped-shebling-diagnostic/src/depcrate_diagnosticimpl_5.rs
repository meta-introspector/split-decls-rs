// Generated macro for impl_5 (impl)
macro_rules! Depcrate_diagnosticimpl_5 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_5"}
// Dependencies: {}
impl Diagnostic { # [doc = " Creates a new [builder](DiagnosticBuilder) for a diagnostic"] # [doc = " of the given [kind](DiagnosticKind)."] pub fn builder (kind : DiagnosticKind) -> DiagnosticBuilder { let help = (& kind as & dyn miette :: Diagnostic) . help () . map (| help | help . to_string ()) ; DiagnosticBuilder { kind , labels : vec ! [] , help , } } # [doc = " The [miette::SourceSpan] of the first label, which is used as the diagnostic's"] # [doc = " main span."] pub fn span (& self) -> & miette :: SourceSpan { & self . labels . first () . expect ("Diagnostics should have at least one label.") . inner () } }
};
}
