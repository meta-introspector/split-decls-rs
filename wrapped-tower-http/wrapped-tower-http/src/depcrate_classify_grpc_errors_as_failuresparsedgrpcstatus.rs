// Generated macro for ParsedGrpcStatus (enum)
macro_rules! Depcrate_classify_grpc_errors_as_failuresParsedGrpcStatus {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"ParsedGrpcStatus"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq)] pub (crate) enum ParsedGrpcStatus { Success , NonSuccess (NonZeroI32) , GrpcStatusHeaderMissing , HeaderNotString , HeaderNotInt , }
};
}
