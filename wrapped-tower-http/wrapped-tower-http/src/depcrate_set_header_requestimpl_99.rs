// Generated macro for impl_99 (impl)
macro_rules! Depcrate_set_header_requestimpl_99 {
() => {
// Module: crate::set_header::request
// Provides: {"impl_99"}
// Dependencies: {}
impl < M > SetRequestHeaderLayer < M > { # [doc = " Create a new [`SetRequestHeaderLayer`]."] # [doc = ""] # [doc = " If a previous value exists for the same header, it is removed and replaced with the new"] # [doc = " header value."] pub fn overriding (header_name : HeaderName , make : M) -> Self { Self :: new (header_name , make , InsertHeaderMode :: Override) } # [doc = " Create a new [`SetRequestHeaderLayer`]."] # [doc = ""] # [doc = " The new header is always added, preserving any existing values. If previous values exist,"] # [doc = " the header will have multiple values."] pub fn appending (header_name : HeaderName , make : M) -> Self { Self :: new (header_name , make , InsertHeaderMode :: Append) } # [doc = " Create a new [`SetRequestHeaderLayer`]."] # [doc = ""] # [doc = " If a previous value exists for the header, the new value is not inserted."] pub fn if_not_present (header_name : HeaderName , make : M) -> Self { Self :: new (header_name , make , InsertHeaderMode :: IfNotPresent) } fn new (header_name : HeaderName , make : M , mode : InsertHeaderMode) -> Self { Self { make , header_name , mode , } } }
};
}
