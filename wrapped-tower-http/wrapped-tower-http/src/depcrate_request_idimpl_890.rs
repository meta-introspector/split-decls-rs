// Generated macro for impl_890 (impl)
macro_rules! Depcrate_request_idimpl_890 {
() => {
// Module: crate::request_id
// Provides: {"impl_890"}
// Dependencies: {}
impl RequestId { # [doc = " Create a new `RequestId` from a [`HeaderValue`]."] pub fn new (header_value : HeaderValue) -> Self { Self (header_value) } # [doc = " Gets a reference to the underlying [`HeaderValue`]."] pub fn header_value (& self) -> & HeaderValue { & self . 0 } # [doc = " Consumes `self`, returning the underlying [`HeaderValue`]."] pub fn into_header_value (self) -> HeaderValue { self . 0 } }
};
}
