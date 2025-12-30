// Generated macro for impl_529 (impl)
macro_rules! Depcrate_trace_on_eosimpl_529 {
() => {
// Module: crate::trace::on_eos
// Provides: {"impl_529"}
// Dependencies: {}
impl OnEos for DefaultOnEos { fn on_eos (self , trailers : Option < & HeaderMap > , stream_duration : Duration , _span : & Span) { let stream_duration = Latency { unit : self . latency_unit , duration : stream_duration , } ; let status = trailers . and_then (| trailers | { match crate :: classify :: grpc_errors_as_failures :: classify_grpc_metadata (trailers , crate :: classify :: GrpcCode :: Ok . into_bitmask () ,) { ParsedGrpcStatus :: Success | ParsedGrpcStatus :: HeaderNotString | ParsedGrpcStatus :: HeaderNotInt => Some (0) , ParsedGrpcStatus :: NonSuccess (status) => Some (status . get ()) , ParsedGrpcStatus :: GrpcStatusHeaderMissing => None , } }) ; event_dynamic_lvl ! (self . level , % stream_duration , status , "end of stream") ; } }
};
}
