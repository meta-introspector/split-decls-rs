// Generated macro for impl_896 (impl)
macro_rules! Depcrate_request_idimpl_896 {
() => {
// Module: crate::request_id
// Provides: {"impl_896"}
// Dependencies: {}
impl < S , M > SetRequestId < S , M > { # [doc = " Create a new `SetRequestId`."] pub fn new (inner : S , header_name : HeaderName , make_request_id : M) -> Self where M : MakeRequestId , { Self { inner , header_name , make_request_id , } } # [doc = " Create a new `SetRequestId` that uses `x-request-id` as the header name."] pub fn x_request_id (inner : S , make_request_id : M) -> Self where M : MakeRequestId , { Self :: new (inner , X_REQUEST_ID , make_request_id) } define_inner_service_accessors ! () ; # [doc = " Returns a new [`Layer`] that wraps services with a `SetRequestId` middleware."] pub fn layer (header_name : HeaderName , make_request_id : M) -> SetRequestIdLayer < M > where M : MakeRequestId , { SetRequestIdLayer :: new (header_name , make_request_id) } }
};
}
