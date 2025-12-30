// Generated macro for impl_503 (impl)
macro_rules! Depcrate_trace_make_spanimpl_503 {
() => {
// Module: crate::trace::make_span
// Provides: {"impl_503"}
// Dependencies: {}
impl < F , B > MakeSpan < B > for F where F : FnMut (& Request < B >) -> Span , { fn make_span (& mut self , request : & Request < B >) -> Span { self (request) } }
};
}
