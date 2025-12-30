// Generated macro for GrpcErrorsAsFailures (struct)
macro_rules! Depcrate_classify_grpc_errors_as_failuresGrpcErrorsAsFailures {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"GrpcErrorsAsFailures"}
// Dependencies: {}
# [doc = " Response classifier for gRPC responses."] # [doc = ""] # [doc = " gRPC doesn't use normal HTTP statuses for indicating success or failure but instead a special"] # [doc = " header that might appear in a trailer."] # [doc = ""] # [doc = " Responses are considered successful if"] # [doc = ""] # [doc = " - `grpc-status` header value contains a success value."] # [doc = " - `grpc-status` header is missing."] # [doc = " - `grpc-status` header value isn't a valid `String`."] # [doc = " - `grpc-status` header value can't parsed into an `i32`."] # [doc = ""] # [doc = " All others are considered failures."] # [derive (Debug , Clone)] pub struct GrpcErrorsAsFailures { success_codes : GrpcCodeBitmask , }
};
}
