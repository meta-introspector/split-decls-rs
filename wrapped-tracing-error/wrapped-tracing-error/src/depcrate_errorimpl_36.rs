// Generated macro for impl_36 (impl)
macro_rules! Depcrate_errorimpl_36 {
() => {
// Module: crate::error
// Provides: {"impl_36"}
// Dependencies: {}
impl ExtractSpanTrace for dyn Error + 'static { fn span_trace (& self) -> Option < & SpanTrace > { self . downcast_ref :: < ErrorImpl < Erased > > () . map (| inner | & inner . span_trace) } }
};
}
