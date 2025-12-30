// Generated macro for SetResponseHeader (struct)
macro_rules! Depcrate_set_header_responseSetResponseHeader {
() => {
// Module: crate::set_header::response
// Provides: {"SetResponseHeader"}
// Dependencies: {}
# [doc = " Middleware that sets a header on the response."] # [derive (Clone)] pub struct SetResponseHeader < S , M > { inner : S , header_name : HeaderName , make : M , mode : InsertHeaderMode , }
};
}
