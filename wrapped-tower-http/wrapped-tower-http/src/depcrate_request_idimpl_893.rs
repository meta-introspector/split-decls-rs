// Generated macro for impl_893 (impl)
macro_rules! Depcrate_request_idimpl_893 {
() => {
// Module: crate::request_id
// Provides: {"impl_893"}
// Dependencies: {}
impl < M > SetRequestIdLayer < M > { # [doc = " Create a new `SetRequestIdLayer`."] pub fn new (header_name : HeaderName , make_request_id : M) -> Self where M : MakeRequestId , { SetRequestIdLayer { header_name , make_request_id , } } # [doc = " Create a new `SetRequestIdLayer` that uses `x-request-id` as the header name."] pub fn x_request_id (make_request_id : M) -> Self where M : MakeRequestId , { SetRequestIdLayer :: new (X_REQUEST_ID , make_request_id) } }
};
}
