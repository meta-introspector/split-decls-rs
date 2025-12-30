// Generated macro for impl_83 (impl)
macro_rules! Depcrate_diagnosticimpl_83 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_83"}
// Dependencies: {}
impl miette :: Diagnostic for ParseDiagnostic { fn code < 'a > (& 'a self) -> Option < Box < dyn fmt :: Display + 'a > > { self . kind . code () } fn severity (& self) -> Option < miette :: Severity > { Some (miette :: Severity :: Warning) } fn help < 'a > (& 'a self) -> Option < Box < dyn fmt :: Display + 'a > > { self . help . as_deref () . map (| help | Box :: new (help) as Box < dyn fmt :: Display >) } fn labels (& self) -> Option < Box < dyn Iterator < Item = miette :: LabeledSpan > + '_ > > { Some (Box :: new (self . labels . iter () . map (| (label , range) | { miette :: LabeledSpan :: new_with_span (Some (label . into ()) , * range) }))) } }
};
}
