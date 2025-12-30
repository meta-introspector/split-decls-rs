// Generated macro for impl_549 (impl)
macro_rules! Depcrate_trace_on_requestimpl_549 {
() => {
// Module: crate::trace::on_request
// Provides: {"impl_549"}
// Dependencies: {}
impl < B , F > OnRequest < B > for F where F : FnMut (& Request < B > , & Span) , { fn on_request (& mut self , request : & Request < B > , span : & Span) { self (request , span) } }
};
}
