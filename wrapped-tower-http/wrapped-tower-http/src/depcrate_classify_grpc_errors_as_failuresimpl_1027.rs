// Generated macro for impl_1027 (impl)
macro_rules! Depcrate_classify_grpc_errors_as_failuresimpl_1027 {
() => {
// Module: crate::classify::grpc_errors_as_failures
// Provides: {"impl_1027"}
// Dependencies: {}
impl GrpcErrorsAsFailures { # [doc = " Create a new [`GrpcErrorsAsFailures`]."] pub fn new () -> Self { Self { success_codes : GrpcCodeBitmask :: OK , } } # [doc = " Change which gRPC codes are considered success."] # [doc = ""] # [doc = " Defaults to only considering `Ok` as success."] # [doc = ""] # [doc = " `Ok` will always be considered a success."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Servers might not want to consider `Invalid Argument` or `Not Found` as failures since"] # [doc = " thats likely the clients fault:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use tower_http::classify::{GrpcErrorsAsFailures, GrpcCode};"] # [doc = ""] # [doc = " let classifier = GrpcErrorsAsFailures::new()"] # [doc = "     .with_success(GrpcCode::InvalidArgument)"] # [doc = "     .with_success(GrpcCode::NotFound);"] # [doc = " ```"] pub fn with_success (mut self , code : GrpcCode) -> Self { self . success_codes |= code . into_bitmask () ; self } # [doc = " Returns a [`MakeClassifier`](super::MakeClassifier) that produces `GrpcErrorsAsFailures`."] # [doc = ""] # [doc = " This is a convenience function that simply calls `SharedClassifier::new`."] pub fn make_classifier () -> SharedClassifier < Self > { SharedClassifier :: new (Self :: new ()) } }
};
}
