// Generated macro for impl_1028 (impl)
macro_rules! Depcrate_classify_grpc_errors_as_failuresimpl_1028 {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"impl_1028"}
// Dependencies: {}
impl ClassifyResponse for GrpcErrorsAsFailures { type FailureClass = GrpcFailureClass ; type ClassifyEos = GrpcEosErrorsAsFailures ; fn classify_response < B > (self , res : & Response < B > ,) -> ClassifiedResponse < Self :: FailureClass , Self :: ClassifyEos > { match classify_grpc_metadata (res . headers () , self . success_codes) { ParsedGrpcStatus :: Success | ParsedGrpcStatus :: HeaderNotString | ParsedGrpcStatus :: HeaderNotInt => ClassifiedResponse :: Ready (Ok (())) , ParsedGrpcStatus :: NonSuccess (status) => { ClassifiedResponse :: Ready (Err (GrpcFailureClass :: Code (status))) } ParsedGrpcStatus :: GrpcStatusHeaderMissing => { ClassifiedResponse :: RequiresEos (GrpcEosErrorsAsFailures { success_codes : self . success_codes , }) } } } fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : fmt :: Display + 'static , { GrpcFailureClass :: Error (error . to_string ()) } }
};
}
