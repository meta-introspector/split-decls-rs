// Generated macro for impl_318 (impl)
macro_rules! Depcrate_profilingimpl_318 {
() => {
// Module: crate::profiling
// Provides: {"impl_318"}
// Dependencies: {}
impl SpannedEventArgRecorder for EventArgRecorder < '_ > { fn record_arg_with_span < A > (& mut self , source_map : & SourceMap , event_arg : A , span : crate :: Span) where A : Borrow < str > + Into < String > , { self . record_arg (event_arg) ; self . record_arg (source_map . span_to_embeddable_string (span)) ; } }
};
}
