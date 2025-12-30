// Generated macro for GrpcFailureClass (enum)
macro_rules! Depcrate_classify_grpc_errors_as_failuresGrpcFailureClass {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"GrpcFailureClass"}
// Dependencies: {}
# [doc = " The failure class for [`GrpcErrorsAsFailures`]."] # [derive (Debug)] pub enum GrpcFailureClass { # [doc = " A gRPC response was classified as a failure with the corresponding status."] Code (std :: num :: NonZeroI32) , # [doc = " A gRPC response was classified as an error with the corresponding error description."] Error (String) , }
};
}
