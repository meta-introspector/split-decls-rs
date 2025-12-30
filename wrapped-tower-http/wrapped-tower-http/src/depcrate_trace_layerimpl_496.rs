// Generated macro for impl_496 (impl)
macro_rules! Depcrate_trace_layerimpl_496 {
() => {
// Module: crate::trace::layer
// Provides: {"impl_496"}
// Dependencies: {}
impl < S , M , MakeSpan , OnRequest , OnResponse , OnBodyChunk , OnEos , OnFailure > Layer < S > for TraceLayer < M , MakeSpan , OnRequest , OnResponse , OnBodyChunk , OnEos , OnFailure > where M : Clone , MakeSpan : Clone , OnRequest : Clone , OnResponse : Clone , OnEos : Clone , OnBodyChunk : Clone , OnFailure : Clone , { type Service = Trace < S , M , MakeSpan , OnRequest , OnResponse , OnBodyChunk , OnEos , OnFailure > ; fn layer (& self , inner : S) -> Self :: Service { Trace { inner , make_classifier : self . make_classifier . clone () , make_span : self . make_span . clone () , on_request : self . on_request . clone () , on_eos : self . on_eos . clone () , on_body_chunk : self . on_body_chunk . clone () , on_response : self . on_response . clone () , on_failure : self . on_failure . clone () , } } }
};
}
