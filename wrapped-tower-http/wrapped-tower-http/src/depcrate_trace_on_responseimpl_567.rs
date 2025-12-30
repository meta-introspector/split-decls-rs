// Generated macro for impl_567 (impl)
macro_rules! Depcrate_trace_on_responseimpl_567 {
() => {
// Module: crate::trace::on_response
// Provides: {"impl_567"}
// Dependencies: {}
impl < B > OnResponse < B > for DefaultOnResponse { fn on_response (self , response : & Response < B > , latency : Duration , _ : & Span) { let latency = Latency { unit : self . latency_unit , duration : latency , } ; let response_headers = self . include_headers . then (| | tracing :: field :: debug (response . headers ())) ; event_dynamic_lvl ! (self . level , % latency , status = status (response) , response_headers , "finished processing request") ; } }
};
}
