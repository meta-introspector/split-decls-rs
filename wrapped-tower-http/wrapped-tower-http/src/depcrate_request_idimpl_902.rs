// Generated macro for impl_902 (impl)
macro_rules! Depcrate_request_idimpl_902 {
() => {
// Module: crate::request_id
// Provides: {"impl_902"}
// Dependencies: {}
impl < S > PropagateRequestId < S > { # [doc = " Create a new `PropagateRequestId`."] pub fn new (inner : S , header_name : HeaderName) -> Self { Self { inner , header_name } } # [doc = " Create a new `PropagateRequestId` that uses `x-request-id` as the header name."] pub fn x_request_id (inner : S) -> Self { Self :: new (inner , X_REQUEST_ID) } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `PropagateRequestId` middleware."] pub fn layer (header_name : HeaderName) -> PropagateRequestIdLayer { PropagateRequestIdLayer :: new (header_name) } }
};
}
