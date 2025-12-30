// Generated macro for impl_103 (impl)
macro_rules! Depcrate_diagnostics_utilsimpl_103 {
() => {
// Module: crate::diagnostics::utils
// Provides: {"impl_103"}
// Dependencies: {}
impl < T > SetOnce < T > for SpannedOption < T > { fn set_once (& mut self , value : T , span : Span) { match self { None => { * self = Some ((value , span)) ; } Some ((_ , prev_span)) => { span_err (span , "attribute specified multiple times") . span_note (* prev_span , "previously specified here") . emit () ; } } } fn value (self) -> Option < T > { self . map (| (v , _) | v) } fn value_ref (& self) -> Option < & T > { self . as_ref () . map (| (v , _) | v) } }
};
}
