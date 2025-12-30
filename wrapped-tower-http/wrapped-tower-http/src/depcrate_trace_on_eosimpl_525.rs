// Generated macro for impl_525 (impl)
macro_rules! Depcrate_trace_on_eosimpl_525 {
() => {
// Module: crate::trace::on_eos
// Provides: {"impl_525"}
// Dependencies: {}
impl < F > OnEos for F where F : FnOnce (Option < & HeaderMap > , Duration , & Span) , { fn on_eos (self , trailers : Option < & HeaderMap > , stream_duration : Duration , span : & Span) { self (trailers , stream_duration , span) } }
};
}
