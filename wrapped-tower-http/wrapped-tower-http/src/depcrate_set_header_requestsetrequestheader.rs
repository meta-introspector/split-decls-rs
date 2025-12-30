// Generated macro for SetRequestHeader (struct)
macro_rules! Depcrate_set_header_requestSetRequestHeader {
() => {
// Module: crate::set_header::request
// Provides: {"SetRequestHeader"}
// Dependencies: {}
# [doc = " Middleware that sets a header on the request."] # [derive (Clone)] pub struct SetRequestHeader < S , M > { inner : S , header_name : HeaderName , make : M , mode : InsertHeaderMode , }
};
}
