// Generated macro for SetResponseHeaderLayer (struct)
macro_rules! Depcrate_set_header_responseSetResponseHeaderLayer {
() => {
// Module: crate::set_header::response
// Provides: {"SetResponseHeaderLayer"}
// Dependencies: {}
# [doc = " Layer that applies [`SetResponseHeader`] which adds a response header."] # [doc = ""] # [doc = " See [`SetResponseHeader`] for more details."] pub struct SetResponseHeaderLayer < M > { header_name : HeaderName , make : M , mode : InsertHeaderMode , }
};
}
