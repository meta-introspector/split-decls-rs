// Generated macro for impl_553 (impl)
macro_rules! Depcrate_trace_on_requestimpl_553 {
() => {
// Module: crate::trace::on_request
// Provides: {"impl_553"}
// Dependencies: {}
impl < B > OnRequest < B > for DefaultOnRequest { fn on_request (& mut self , _ : & Request < B > , _ : & Span) { event_dynamic_lvl ! (self . level , "started processing request") ; } }
};
}
