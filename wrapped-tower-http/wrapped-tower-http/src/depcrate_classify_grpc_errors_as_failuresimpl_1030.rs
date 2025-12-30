// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_classify_grpc_errors_as_failuresimpl_1030 {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"impl_1030"}
// Dependencies: {}
impl ClassifyEos for GrpcEosErrorsAsFailures { type FailureClass = GrpcFailureClass ; fn classify_eos (self , trailers : Option < & HeaderMap >) -> Result < () , Self :: FailureClass > { if let Some (trailers) = trailers { match classify_grpc_metadata (trailers , self . success_codes) { ParsedGrpcStatus :: Success | ParsedGrpcStatus :: GrpcStatusHeaderMissing | ParsedGrpcStatus :: HeaderNotString | ParsedGrpcStatus :: HeaderNotInt => Ok (()) , ParsedGrpcStatus :: NonSuccess (status) => Err (GrpcFailureClass :: Code (status)) , } } else { Ok (()) } } fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : fmt :: Display + 'static , { GrpcFailureClass :: Error (error . to_string ()) } }
};
}
