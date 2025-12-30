// Generated macro for impl_563 (impl)
macro_rules! Depcrate_trace_on_responseimpl_563 {
() => {
// Module: crate::trace::on_response
// Provides: {"impl_563"}
// Dependencies: {}
impl < B , F > OnResponse < B > for F where F : FnOnce (& Response < B > , Duration , & Span) , { fn on_response (self , response : & Response < B > , latency : Duration , span : & Span) { self (response , latency , span) } }
};
}
