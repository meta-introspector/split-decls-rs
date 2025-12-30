// Generated macro for impl_6 (impl)
macro_rules! Depcrate_diagnosticimpl_6 {
() => {
// Module: crate::diagnostic
// Provides: {"impl_6"}
// Dependencies: {}
impl miette :: Diagnostic for Diagnostic { fn code < 'a > (& 'a self) -> Option < Box < dyn fmt :: Display + 'a > > { self . kind . code () } fn severity (& self) -> Option < miette :: Severity > { self . kind . severity () . or (Some (miette :: Severity :: Warning)) } fn help < 'a > (& 'a self) -> Option < Box < dyn fmt :: Display + 'a > > { self . help . as_deref () . map (| help | Box :: new (help) as Box < dyn fmt :: Display >) } fn labels (& self) -> Option < Box < dyn Iterator < Item = miette :: LabeledSpan > + '_ > > { Some (Box :: new (self . labels . clone () . into_iter ())) } }
};
}
