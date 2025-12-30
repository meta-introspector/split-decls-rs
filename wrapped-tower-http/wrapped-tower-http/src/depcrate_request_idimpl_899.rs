// Generated macro for impl_899 (impl)
macro_rules! Depcrate_request_idimpl_899 {
() => {
// Module: crate::request_id
// Provides: {"impl_899"}
// Dependencies: {}
impl PropagateRequestIdLayer { # [doc = " Create a new `PropagateRequestIdLayer`."] pub fn new (header_name : HeaderName) -> Self { PropagateRequestIdLayer { header_name } } # [doc = " Create a new `PropagateRequestIdLayer` that uses `x-request-id` as the header name."] pub fn x_request_id () -> Self { Self :: new (X_REQUEST_ID) } }
};
}
