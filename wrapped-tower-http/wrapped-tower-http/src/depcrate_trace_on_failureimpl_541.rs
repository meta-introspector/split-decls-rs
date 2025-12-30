// Generated macro for impl_541 (impl)
macro_rules! Depcrate_trace_on_failureimpl_541 {
() => {
// Module: crate::trace::on_failure
// Provides: {"impl_541"}
// Dependencies: {}
impl < FailureClass > OnFailure < FailureClass > for DefaultOnFailure where FailureClass : fmt :: Display , { fn on_failure (& mut self , failure_classification : FailureClass , latency : Duration , _ : & Span) { let latency = Latency { unit : self . latency_unit , duration : latency , } ; event_dynamic_lvl ! (self . level , classification = % failure_classification , % latency , "response failed") ; } }
};
}
