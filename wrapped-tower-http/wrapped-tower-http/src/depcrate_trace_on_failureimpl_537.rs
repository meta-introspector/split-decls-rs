// Generated macro for impl_537 (impl)
macro_rules! Depcrate_trace_on_failureimpl_537 {
() => {
// Module: crate::trace::on_failure
// Provides: {"impl_537"}
// Dependencies: {}
impl < F , FailureClass > OnFailure < FailureClass > for F where F : FnMut (FailureClass , Duration , & Span) , { fn on_failure (& mut self , failure_classification : FailureClass , latency : Duration , span : & Span) { self (failure_classification , latency , span) } }
};
}
