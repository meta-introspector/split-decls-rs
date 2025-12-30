// Generated macro for impl_103 (impl)
macro_rules! Depcrate_set_header_requestimpl_103 {
() => {
// Module: crate::set_header::request
// Provides: {"impl_103"}
// Dependencies: {}
impl < S , M > SetRequestHeader < S , M > { # [doc = " Create a new [`SetRequestHeader`]."] # [doc = ""] # [doc = " If a previous value exists for the same header, it is removed and replaced with the new"] # [doc = " header value."] pub fn overriding (inner : S , header_name : HeaderName , make : M) -> Self { Self :: new (inner , header_name , make , InsertHeaderMode :: Override) } # [doc = " Create a new [`SetRequestHeader`]."] # [doc = ""] # [doc = " The new header is always added, preserving any existing values. If previous values exist,"] # [doc = " the header will have multiple values."] pub fn appending (inner : S , header_name : HeaderName , make : M) -> Self { Self :: new (inner , header_name , make , InsertHeaderMode :: Append) } # [doc = " Create a new [`SetRequestHeader`]."] # [doc = ""] # [doc = " If a previous value exists for the header, the new value is not inserted."] pub fn if_not_present (inner : S , header_name : HeaderName , make : M) -> Self { Self :: new (inner , header_name , make , InsertHeaderMode :: IfNotPresent) } fn new (inner : S , header_name : HeaderName , make : M , mode : InsertHeaderMode) -> Self { Self { inner , header_name , make , mode , } } define_inner_service_accessors ! () ; }
};
}
